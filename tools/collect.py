# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

"""
NuttX crate collection utility.

This module provides a Collector class to collect crate directories that
contain the required files for NuttX integration.
"""

import os
import re


class Collector:
    """
    A class to collect and manage NuttX crate directories.
    """

    def __init__(self, base_path):
        """
        Initialize the Collector with a base path.

        Args:
            base_path (str): Base path to start the search from
        """
        self.base_path = base_path
        self.collect_crate_dirs()

    def collect_crate_dirs(self):
        """
        Find directories that contain all three required files:
        Cargo.toml, CMakeLists.txt, and Kconfig.

        Returns:
            list: List of directories containing all required files
        """
        self.crate_dirs = []
        for root, _, files in os.walk(self.base_path):
            if (
                "Cargo.toml" in files
                and "CMakeLists.txt" in files
                and "Kconfig" in files
            ):
                self.crate_dirs.append(root)
        return self.crate_dirs

    def get_kconfig_main_config(self, kconfig_path):
        """
        Extract the main CONFIG option from a Kconfig file using regex.

        Looks for config entries that start with RUST_, supporting various
        patterns like RUST_CRATE_*, RUST_STD_*, RUST_CORE_*, etc.

        Args:
            kconfig_path (str): Path to the Kconfig file

        Returns:
            str: The main CONFIG option name or None if not found
        """
        try:
            with open(kconfig_path, "r") as f:
                content = f.read()
                # Match 'config RUST_*' pattern
                pattern = r"config\s+(RUST_[A-Za-z0-9_]+)"
                matches = re.search(pattern, content)
                if matches:
                    return matches.group(1)
        except Exception as e:
            print(f"Error reading Kconfig file {kconfig_path}: {e}")
        return None

    def get_crate_config_mapping(self):
        """
        Get a mapping of crate directories to their main CONFIG options.

        Returns:
            dict: Dictionary mapping crate directories to their main CONFIG options

        Example:
            {
                '/home/Work/nuttx-crates-index/crates/serde_json': 'CONFIG_RUST_CRATE_SERDE_JSON',
                '/home/Work/nuttx-crates-index/crates/std_thread': 'CONFIG_RUST_CRATE_STD_THREAD',
                '/home/Work/nuttx-crates-index/crates/rand': 'CONFIG_RUST_CRATE_RAND',
                '/home/Work/nuttx-crates-index/crates/std_println': 'CONFIG_RUST_CRATE_STD_PRINTLN',
                '/home/Work/nuttx-crates-index/crates/core_println': 'CONFIG_RUST_CRATE_CORE_PRINTLN',
                '/home/Work/nuttx-crates-index/crates/lazy_static': 'CONFIG_RUST_CRATE_LAZY_STATIC',
                '/home/Work/nuttx-crates-index/crates/regex_lite': 'CONFIG_RUST_CRATE_REGEX_LITE',
                '/home/Work/nuttx-crates-index/crates/once_cell': 'CONFIG_RUST_CRATE_ONCE_CELL',
                '/home/Work/nuttx-crates-index/crates/regex': 'CONFIG_RUST_CRATE_REGEX'
            }
        """
        configs = {}
        for crate_dir in self.crate_dirs:
            kconfig_path = os.path.join(crate_dir, "Kconfig")
            if os.path.exists(kconfig_path):
                config = self.get_kconfig_main_config(kconfig_path)
                if config:
                    configs[crate_dir] = f"CONFIG_{config}"
        return configs
