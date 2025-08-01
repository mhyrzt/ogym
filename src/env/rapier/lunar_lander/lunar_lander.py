__credits__ = ["Andrea PIERRÉ"]

import math
from typing import TYPE_CHECKING

import numpy as np

import gymnasium as gym
from gymnasium import error, spaces
from gymnasium.error import DependencyNotInstalled
from gymnasium.utils import EzPickle

try:
    import Box2D
    from Box2D.b2 import (
        circleShape,
        contactListener,
        edgeShape,
        fixtureDef,
        polygonShape,
        revoluteJointDef,
    )
except ImportError as e:
    raise DependencyNotInstalled(
        'Box2D is not installed, you can install it by run `pip install swig` followed by `pip install "gymnasium[box2d]"`'
    ) from e

LANDER_POLY = [(-14, +17), (-17, 0), (-17, -10), (+17, -10), (+17, 0), (+14, +17)]


class LunarLander(gym.Env, EzPickle):
    """
    Lunar Lander environment with discrete or continuous action space.

    The goal is to land the spacecraft between the flags on the landing pad
    while minimizing fuel consumption and avoiding crashes.
    """

    metadata = {
        "render_modes": ["human", "rgb_array"],
        "render_fps": 50,
    }

    def __init__(
        self,
        continuous: bool = False,
        gravity: float = -10.0,
        enable_wind: bool = False,
        wind_power: float = 15.0,
        turbulence_power: float = 1.5,
        scale: float = 30.0,
        fps: int = 50,
        main_engine_power: float = 13.0,
        side_engine_power: float = 0.6,
        initial_random: float = 1000.0,
        leg_away: float = 20,
        leg_down: float = 18,
        leg_width: float = 2,
        leg_height: float = 8,
        leg_spring_torque: float = 40,
        side_engine_height: float = 14,
        side_engine_away: float = 12,
        main_engine_y_location: float = 4,
        viewport_width: int = 600,
        viewport_height: int = 400,
    ):
        EzPickle.__init__(
            self,
            continuous,
            gravity,
            enable_wind,
            wind_power,
            turbulence_power,
        )

        assert (
            -12.0 < gravity < 0.0
        ), f"gravity must be between -12 and 0, got {gravity}"

        if not (0.0 <= wind_power <= 20.0):
            gym.logger.warn(
                f"wind_power recommended between 0.0-20.0, got {wind_power}"
            )

        if not (0.0 <= turbulence_power <= 2.0):
            gym.logger.warn(
                f"turbulence_power recommended between 0.0-2.0, got {turbulence_power}"
            )

        # Physics parameters
        self.gravity = gravity
        self.wind_power = wind_power
        self.turbulence_power = turbulence_power
        self.enable_wind = enable_wind
        self.scale = scale
        self.fps = fps

        # Engine parameters
        self.main_engine_power = main_engine_power
        self.side_engine_power = side_engine_power

        # Lander parameters
        self.initial_random = initial_random

        self.leg_away = leg_away
        self.leg_down = leg_down
        self.leg_width = leg_width
        self.leg_height = leg_height
        self.leg_spring_torque = leg_spring_torque
        self.side_engine_height = side_engine_height
        self.side_engine_away = side_engine_away
        self.main_engine_y_location = main_engine_y_location

        # Viewport parameters
        self.viewport_width = viewport_width
        self.viewport_height = viewport_height

        # Environment state
        self.continuous = continuous
        self.world = Box2D.b2World(gravity=(0, gravity))
        self.moon = None
        self.lander = None
        self.legs = []
        self.particles = []
        self.game_over = False
        self.prev_shaping = None
        self.wind_idx = 0
        self.torque_idx = 0

        # Set up contact detection
        self._setup_contact_detection()

        # Define observation and action spaces
        self._setup_spaces()

    def _setup_contact_detection(self):
        """Initialize contact detection system"""

        class ContactDetector(contactListener):
            def __init__(self, env):
                contactListener.__init__(self)
                self.env = env

            def BeginContact(self, contact):
                if (
                    self.env.lander == contact.fixtureA.body
                    or self.env.lander == contact.fixtureB.body
                ):
                    self.env.game_over = True

                for i in range(2):
                    if self.env.legs[i] in [
                        contact.fixtureA.body,
                        contact.fixtureB.body,
                    ]:
                        self.env.legs[i].ground_contact = True

            def EndContact(self, contact):
                for i in range(2):
                    if self.env.legs[i] in [
                        contact.fixtureA.body,
                        contact.fixtureB.body,
                    ]:
                        self.env.legs[i].ground_contact = False

        self.world.contactListener_keepref = ContactDetector(self)
        self.world.contactListener = self.world.contactListener_keepref

    def _setup_spaces(self):
        """Define observation and action spaces"""
        low = np.array(
            [
                -2.5,
                -2.5,  # position bounds
                -10.0,
                -10.0,  # velocity bounds
                -2 * math.pi,
                -10.0,  # angle and angular velocity
                0.0,
                0.0,  # leg contact
            ]
        ).astype(np.float32)

        high = np.array(
            [
                2.5,
                2.5,  # position bounds
                10.0,
                10.0,  # velocity bounds
                2 * math.pi,
                10.0,  # angle and angular velocity
                1.0,
                1.0,  # leg contact
            ]
        ).astype(np.float32)

        self.observation_space = spaces.Box(low, high)

        if self.continuous:
            self.action_space = spaces.Box(-1, +1, (2,), dtype=np.float32)
        else:
            self.action_space = spaces.Discrete(4)

    def _destroy(self):
        """Clean up the physics world"""
        if not self.moon:
            return

        self.world.contactListener = None
        self._clean_particles(True)

        if self.moon:
            self.world.DestroyBody(self.moon)
            self.moon = None

        if self.lander:
            self.world.DestroyBody(self.lander)
            self.lander = None

        for leg in self.legs:
            self.world.DestroyBody(leg)
        self.legs = []

    def _create_terrain(self):
        """Create the lunar surface terrain"""
        W = self.viewport_width / self.scale
        H = self.viewport_height / self.scale

        CHUNKS = 11
        height = self.np_random.uniform(0, H / 2, size=(CHUNKS + 1,))
        chunk_x = [W / (CHUNKS - 1) * i for i in range(CHUNKS)]

        self.helipad_x1 = chunk_x[CHUNKS // 2 - 1]
        self.helipad_x2 = chunk_x[CHUNKS // 2 + 1]
        self.helipad_y = H / 4

        # Create flat landing pad
        for i in range(CHUNKS // 2 - 2, CHUNKS // 2 + 3):
            height[i] = self.helipad_y

        smooth_y = [
            0.33 * (height[i - 1] + height[i] + height[i + 1]) for i in range(CHUNKS)
        ]

        self.moon = self.world.CreateStaticBody(
            shapes=edgeShape(vertices=[(0, 0), (W, 0)])
        )

        for i in range(CHUNKS - 1):
            p1 = (chunk_x[i], smooth_y[i])
            p2 = (chunk_x[i + 1], smooth_y[i + 1])
            self.moon.CreateEdgeFixture(vertices=[p1, p2], density=0, friction=0.1)

        self.moon.color1 = (0.0, 0.0, 0.0)
        self.moon.color2 = (0.0, 0.0, 0.0)

    def _create_lander(self):
        """Create the lunar lander spacecraft"""
        initial_y = self.viewport_height / self.scale
        initial_x = self.viewport_width / self.scale / 2

        self.lander = self.world.CreateDynamicBody(
            position=(initial_x, initial_y),
            angle=0.0,
            fixtures=fixtureDef(
                shape=polygonShape(
                    vertices=[
                        (x / self.scale, y / self.scale) for x, y in LANDER_POLY
                    ]
                ),
                density=5.0,
                friction=0.1,
                categoryBits=0x0010,
                maskBits=0x001,
                restitution=0.0,
            ),
        )
        self.lander.color1 = (128, 102, 230)
        self.lander.color2 = (77, 77, 128)

        # Apply initial random impulse
        self.lander.ApplyForceToCenter(
            (
                self.np_random.uniform(-self.initial_random, self.initial_random),
                self.np_random.uniform(-self.initial_random, self.initial_random),
            ),
            True,
        )

    def _create_legs(self):
        """Create the lander's landing legs"""
        initial_y = self.viewport_height / self.scale
        initial_x = self.viewport_width / self.scale / 2

        self.legs = []
        for i in [-1, +1]:
            leg = self.world.CreateDynamicBody(
                position=(initial_x - i * self.leg_away / self.scale, initial_y),
                angle=(i * 0.05),
                fixtures=fixtureDef(
                    shape=polygonShape(
                        box=(self.leg_width / self.scale, self.leg_height / self.scale)
                    ),
                    density=1.0,
                    restitution=0.0,
                    categoryBits=0x0020,
                    maskBits=0x001,
                ),
            )
            leg.ground_contact = False
            leg.color1 = (128, 102, 230)
            leg.color2 = (77, 77, 128)

            rjd = revoluteJointDef(
                bodyA=self.lander,
                bodyB=leg,
                localAnchorA=(0, 0),
                localAnchorB=(
                    i * self.leg_away / self.scale,
                    self.leg_down / self.scale,
                ),
                enableMotor=True,
                enableLimit=True,
                maxMotorTorque=self.leg_spring_torque,
                motorSpeed=+0.3 * i,
            )

            if i == -1:
                rjd.lowerAngle = +0.9 - 0.5
                rjd.upperAngle = +0.9
            else:
                rjd.lowerAngle = -0.9
                rjd.upperAngle = -0.9 + 0.5

            leg.joint = self.world.CreateJoint(rjd)
            self.legs.append(leg)

    def _apply_wind_effects(self):
        """Apply wind and turbulence effects to the lander"""
        if not self.enable_wind or (
            self.legs[0].ground_contact or self.legs[1].ground_contact
        ):
            return

        # Apply wind force
        wind_mag = (
            math.tanh(
                math.sin(0.02 * self.wind_idx)
                + math.sin(math.pi * 0.01 * self.wind_idx)
            )
            * self.wind_power
        )
        self.wind_idx += 1
        self.lander.ApplyForceToCenter((wind_mag, 0.0), True)

        # Apply turbulence torque
        torque_mag = (
            math.tanh(
                math.sin(0.02 * self.torque_idx)
                + math.sin(math.pi * 0.01 * self.torque_idx)
            )
            * self.turbulence_power
        )
        self.torque_idx += 1
        self.lander.ApplyTorque(torque_mag, True)

    def _apply_engine_forces(self, action):
        """Apply forces from main and side engines"""
        if self.continuous:
            action = np.clip(action, -1, +1).astype(np.float64)
        else:
            assert self.action_space.contains(action), f"Invalid action: {action}"

        tip = (math.sin(self.lander.angle), math.cos(self.lander.angle))
        side = (-tip[1], tip[0])
        dispersion = [self.np_random.uniform(-1.0, +1.0) / self.scale for _ in range(2)]

        # Main engine
        m_power = 0.0
        if (self.continuous and action[0] > 0.0) or (
            not self.continuous and action == 2
        ):
            if self.continuous:
                m_power = (np.clip(action[0], 0.0, 1.0) + 1.0) * 0.5
            else:
                m_power = 1.0

            ox = (
                tip[0] * (self.main_engine_y_location / self.scale + 2 * dispersion[0])
                + side[0] * dispersion[1]
            )
            oy = (
                -tip[1] * (self.main_engine_y_location / self.scale + 2 * dispersion[0])
                - side[1] * dispersion[1]
            )

            impulse_pos = (self.lander.position[0] + ox, self.lander.position[1] + oy)
            self.lander.ApplyLinearImpulse(
                (
                    -ox * self.main_engine_power * m_power,
                    -oy * self.main_engine_power * m_power,
                ),
                impulse_pos,
                True,
            )

        # Side engines
        s_power = 0.0
        if (self.continuous and np.abs(action[1]) > 0.5) or (
            not self.continuous and action in [1, 3]
        ):

            if self.continuous:
                direction = np.sign(action[1])
                s_power = np.clip(np.abs(action[1]), 0.5, 1.0)
            else:
                direction = action - 2
                s_power = 1.0

            ox = tip[0] * dispersion[0] + side[0] * (
                3 * dispersion[1] + direction * self.side_engine_away / self.scale
            )
            oy = -tip[1] * dispersion[0] - side[1] * (
                3 * dispersion[1] + direction * self.side_engine_away / self.scale
            )

            impulse_pos = (
                self.lander.position[0] + ox - tip[0] * 17 / self.scale,
                self.lander.position[1]
                + oy
                + tip[1] * self.side_engine_height / self.scale,
            )

            self.lander.ApplyLinearImpulse(
                (
                    -ox * self.side_engine_power * s_power,
                    -oy * self.side_engine_power * s_power,
                ),
                impulse_pos,
                True,
            )

        return m_power, s_power

    def _create_particle(self, mass, x, y, ttl):
        """Create visual particle for engine effects"""
        p = self.world.CreateDynamicBody(
            position=(x, y),
            angle=0.0,
            fixtures=fixtureDef(
                shape=circleShape(radius=2 / self.scale, pos=(0, 0)),
                density=mass,
                friction=0.1,
                categoryBits=0x0100,
                maskBits=0x001,
                restitution=0.3,
            ),
        )
        p.ttl = ttl
        self.particles.append(p)
        self._clean_particles(False)
        return p

    def _clean_particles(self, all_particles):
        """Remove expired particles"""
        while self.particles and (all_particles or self.particles[0].ttl < 0):
            self.world.DestroyBody(self.particles.pop(0))

    def _get_state(self):
        """Get current environment state"""
        pos = self.lander.position
        vel = self.lander.linearVelocity

        state = [
            (pos.x - self.viewport_width / self.scale / 2)
            / (self.viewport_width / self.scale / 2),
            (pos.y - (self.helipad_y + self.leg_down / self.scale))
            / (self.viewport_height / self.scale / 2),
            vel.x * (self.viewport_width / self.scale / 2) / self.fps,
            vel.y * (self.viewport_height / self.scale / 2) / self.fps,
            self.lander.angle,
            20.0 * self.lander.angularVelocity / self.fps,
            1.0 if self.legs[0].ground_contact else 0.0,
            1.0 if self.legs[1].ground_contact else 0.0,
        ]
        return state

    def _calculate_reward(self, state, m_power, s_power):
        """Calculate reward based on current state and actions"""
        shaping = (
            -100 * np.sqrt(state[0] * state[0] + state[1] * state[1])
            - 100 * np.sqrt(state[2] * state[2] + state[3] * state[3])
            - 100 * abs(state[4])
            + 10 * state[6]
            + 10 * state[7]
        )

        reward = 0
        if self.prev_shaping is not None:
            reward = shaping - self.prev_shaping
        self.prev_shaping = shaping

        reward -= m_power * 0.30
        reward -= s_power * 0.03

        return reward

    def reset(self, *, seed: int | None = None, options: dict | None = None):
        """Reset the environment to initial state"""
        super().reset(seed=seed)
        self._destroy()

        self.world = Box2D.b2World(gravity=(0, self.gravity))
        self._setup_contact_detection()
        self.game_over = False
        self.prev_shaping = None

        if self.enable_wind:
            self.wind_idx = self.np_random.integers(-9999, 9999)
            self.torque_idx = self.np_random.integers(-9999, 9999)

        self._create_terrain()
        self._create_lander()
        self._create_legs()

        initial_action = np.array([0, 0]) if self.continuous else 0
        return self._get_state(), {}

    def step(self, action):
        """Execute one environment step"""
        assert self.lander is not None, "Call reset() before step()"

        self._apply_wind_effects()
        m_power, s_power = self._apply_engine_forces(action)

        self.world.Step(1.0 / self.fps, 6 * 30, 2 * 30)

        state = self._get_state()
        reward = self._calculate_reward(state, m_power, s_power)

        terminated = False
        if self.game_over or abs(state[0]) >= 1.0:
            terminated = True
            reward = -100
        elif not self.lander.awake:
            terminated = True
            reward = +100

        return np.array(state, dtype=np.float32), reward, terminated, False, {}

    def render(self):
        """Render is removed as per requirements"""
        pass

    def close(self):
        """Clean up resources"""
        self._destroy()
