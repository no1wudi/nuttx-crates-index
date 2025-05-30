# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

"""
NuttX builder module.

This module provides a Builder class that handles the configuration
and build process for NuttX, including CMake configuration,
Kconfig tweaking, and compilation.
"""

import os
import subprocess
import shutil
import sys
from multiprocessing import cpu_count

# Configuration map using tuples: (action, option, [value])
# For binary options: (action, option)
# For valued options: (action, option, value)
_RUST_CONFIG = [
    ("enable", "CONFIG_SYSTEM_TIME64"),
    ("enable", "CONFIG_FS_LARGEFILE"),
    ("enable", "CONFIG_DEV_URANDOM"),
    ("enable", "CONFIG_DEBUG_FULLOPT"),
    ("enable", "CONFIG_FRAME_POINTER"),
    ("enable", "CONFIG_FS_TMPFS"),
    ("enable", "CONFIG_SCHED_BACKTRACE"),
    ("enable", "CONFIG_MM_BACKTRACE_DEFAULT"),
    ("enable", "CONFIG_FS_PROCFS"),
    ("enable", "CONFIG_SCHED_DUMP_LEAK"),
    # ("enable", "CONFIG_DEBUG_FEATURES"),
    # ("enable", "CONFIG_DEBUG_ASSERTIONS"),
    ("set-val", "CONFIG_TLS_NELEM", "16"),
    ("set-val", "CONFIG_DEFAULT_TASK_STACKSIZE", "4096"),
    ("set-val", "CONFIG_INIT_STACKSIZE", "4096"),
    ("set-val", "CONFIG_MM_BACKTRACE", "16"),
]

# Board-specific configurations
_BOARD_CONFIG = {
    "rv-virt": [
        ("disable", "CONFIG_ARCH_FPU"),
    ],
}


class CommandRunner:
    """
    Utility class for running shell commands safely.

    This class provides a static method to execute shell commands
    and handle errors appropriately.
    """

    @staticmethod
    def run(cmd: str, cwd: str = None) -> int:
        """
        Run a shell command and handle potential errors.

        Args:
            cmd: The command to execute
            cwd: Current working directory for the command (optional)

        Returns:
            Return code of the process

        Raises:
            SystemExit: If the command fails to execute
        """
        try:
            process = subprocess.run(
                cmd,
                cwd=cwd,
                shell=True,
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            return process.returncode
        except subprocess.CalledProcessError as e:
            print(f"Failed to run: {cmd}")
            print(e.stderr)
            sys.exit(1)


class Builder:
    """
    NuttX build manager.

    This class handles the configuration and build process for NuttX,
    including CMake configuration, Kconfig tweaking, and compilation.
    """

    def __init__(self, board_config: str, nuttx_path: str):
        """
        Initialize the NuttX builder.

        Args:
            board_config: The board configuration to use
            nuttx_path: Path to the NuttX source directory
        """
        self.board_config = board_config
        self.board = board_config.split(":")[0]
        self.config = board_config.split(":")[1]
        self.nuttx_path = nuttx_path
        self.build_dir = "build"
        self.runner = CommandRunner()

    def _clean_build_dir(self):
        """
        Clean the build directory by removing it if it exists.
        """
        if os.path.exists(self.build_dir):
            shutil.rmtree(self.build_dir)

    def _configure_cmake(self):
        """
        Configure the build using CMake with the specified board configuration.
        """
        self.runner.run(
            f"cmake -B{self.build_dir} -GNinja -DBOARD_CONFIG={self.board_config} {self.nuttx_path}"
        )

    def _apply_kconfig_option(self, config):
        """
        Apply a single Kconfig option using kconfig-tweak.

        Args:
            config: A tuple representing the config option
                   Format for binary options: (action, option)
                   Format for valued options: (action, option, value)
        """
        for cfg in config:
            if len(cfg) == 2:
                action, option = cfg
                self.runner.run(
                    f"kconfig-tweak --{action} {option}", cwd=self.build_dir
                )
            elif len(cfg) == 3:
                action, option, value = cfg
                self.runner.run(
                    f"kconfig-tweak --{action} {option} {value}", cwd=self.build_dir
                )

    def _configure_kconfig(self, configs=None):
        """
        Configure Kconfig options for the NuttX build.

        Sets essential configuration options required for proper functionality.
        Uses a structured configuration map for better maintainability.

        Args:
            configs: Optional list of additional configuration tuples following the same format as _RUST_CONFIG
        """
        # Apply default configurations
        self._apply_kconfig_option(_RUST_CONFIG)

        # Apply board-specific configurations if available
        if self.board in _BOARD_CONFIG:
            self._apply_kconfig_option(_BOARD_CONFIG[self.board])

        if self.board_config in _BOARD_CONFIG:
            self._apply_kconfig_option(_BOARD_CONFIG[self.board_config])

        # Ensure the board-specific configuration is set
        self.runner.run("ninja olddefconfig", cwd=self.build_dir)

        # Apply the configurations
        if configs:
            self._apply_kconfig_option(configs)

        self.runner.run("ninja olddefconfig", cwd=self.build_dir)

    def configure(self, extra=None):
        """
        Configure the build environment.

        This method performs a complete configuration by:
        1. Cleaning the build directory
        2. Running CMake configuration
        3. Setting Kconfig options

        Args:
            extra: Optional list of additional configuration tuples
                  to be passed to _configure_kconfig
        """
        self._clean_build_dir()
        self._configure_cmake()
        self._configure_kconfig(extra)

    def _parse_size_info(self):
        """
        Parse size information from the built NuttX binary.

        Returns:
            dict: Contains the size information with keys 'text', 'data', 'bss', 'total'
        """
        nuttx_binary = os.path.join(self.build_dir, "nuttx")
        if not os.path.exists(nuttx_binary):
            print(f"Warning: NuttX binary not found at {nuttx_binary}")
            return {"text": 0, "data": 0, "bss": 0, "total": 0}

        try:
            result = subprocess.run(
                f"size {nuttx_binary}",
                shell=True,
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )

            # Parse size output, which typically has format:
            # text    data     bss     dec     hex filename
            # XXXXX   XXXXX   XXXXX   XXXXX   XXXXX nuttx
            lines = result.stdout.strip().split("\n")
            if len(lines) < 2:
                print(
                    f"Warning: Unexpected output format from size command: {result.stdout}"
                )
                return {"text": 0, "data": 0, "bss": 0, "total": 0}

            # Get the second line with actual size values
            size_values = lines[1].split()
            if len(size_values) < 5:
                print(f"Warning: Unexpected size output format: {lines[1]}")
                return {"text": 0, "data": 0, "bss": 0, "total": 0}

            # Extract the size values
            text = int(size_values[0])
            data = int(size_values[1])
            bss = int(size_values[2])
            total = text + data + bss

            return {"text": text, "data": data, "bss": bss, "total": total}
        except subprocess.CalledProcessError as e:
            print(f"Failed to run size command: {e}")
            return {"text": 0, "data": 0, "bss": 0, "total": 0}

    def build(self):
        """
        Build NuttX using the configured environment with Ninja.

        Uses parallel build automatically managed by Ninja and
        reports build time and binary size information.

        Returns:
            dict: Size information of the built binary
                 Example: {'text': 156540, 'data': 1016, 'bss': 27456, 'total': 185012}
        """
        self.runner.run("ninja", cwd=self.build_dir)

        # Get and return size information
        size_info = self._parse_size_info()
        return size_info
