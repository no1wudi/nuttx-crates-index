# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

"""
NuttX build utility script.

This script provides functionality to configure and build NuttX for
specified board configurations. It handles the CMake configuration,
Kconfig tweaking, and build process.
"""

import argparse
import os
from builder import Builder
from collect import Collector


def main():
    """
    Main entry point for the NuttX build script.

    Parses command line arguments and initiates the build process
    with the specified configuration.
    """
    parser = argparse.ArgumentParser(
        description="Build NuttX with specified board configuration"
    )
    parser.add_argument("board_config", help="Board configuration to use")
    parser.add_argument(
        "--path", default="nuttx", help="Path to NuttX directory (default: nuttx)"
    )

    args = parser.parse_args()

    # Get parent dir from the script file location
    script_dir = os.path.dirname(os.path.abspath(__file__))
    parent_dir = os.path.dirname(script_dir)

    # Collect crate directories using the Collector class
    collector = Collector(f"{parent_dir}/crates")
    print(f"Found {len(collector.crate_dirs)} crate directories:")
    for crate_dir in collector.crate_dirs:
        print(f"  - {os.path.basename(crate_dir)}")

    # Get the main Kconfig option from the Kconfig file
    kconfig_option = collector.get_crate_config_mapping()

    builder = Builder(args.board_config, args.path)
    print(f"Building NuttX Baseline")
    builder.configure()
    print(builder.build())

    # Iterate over the crate directories and build each one with their config options
    for crate_path, config_option in kconfig_option.items():
        crate_name = os.path.basename(crate_path)
        print(f"Building crate: {crate_name} with option: {config_option}")
        builder.configure([("enable", config_option)])
        print(builder.build())


if __name__ == "__main__":
    main()
