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
import time
from builder import Builder
from collect import Collector
from result_manager import JsonResultManager
from utils import print_build_results


def main():
    """
    Main entry point for the NuttX build script.

    Parses command line arguments and initiates the build process
    with the specified configuration.
    """
    # Capture timestamp at the beginning of execution
    start_timestamp = int(time.time())

    parser = argparse.ArgumentParser(
        description="Build NuttX with specified board configuration"
    )
    parser.add_argument("board", help="Board configuration to use")
    parser.add_argument(
        "--path", default="nuttx", help="Path to NuttX directory (default: nuttx)"
    )
    parser.add_argument("--json", help="JSON file to append build results to")

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

    # Use the nsh config for each board
    builder = Builder(f"{args.board}:nsh", args.path)
    print(f"Building NuttX Baseline")
    builder.configure()
    baseline = builder.build()
    print(
        f"Baseline size: text={baseline['text']} data={baseline['data']} bss={baseline['bss']} total={baseline['total']}"
    )

    # Create JSON result manager (will discard data if args.json is None)
    json_manager = JsonResultManager(args.json)

    # Iterate over the crate directories and build each one with their config options
    for crate_path, config_option in kconfig_option.items():
        crate_name = os.path.basename(crate_path)
        print(f"\nBuilding crate: {crate_name} with option: {config_option}")

        start_time = time.time()
        builder.configure([("enable", config_option)])
        crate_size = builder.build()
        build_time = time.time() - start_time

        # Print build results and get size differences
        diffs = print_build_results(crate_name, baseline, crate_size, build_time)

        # Always collect results, but only mention file writing when args.json is provided
        json_manager.append_result(
            crate_name,
            baseline,
            crate_size,
            diffs,
            start_timestamp,
        )

    json_manager.flush()  # Will only write to file if args.json was provided
    print(f"💾 Results to {args.json}")


if __name__ == "__main__":
    main()
