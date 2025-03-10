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
from runner import Runner
from utils import print_build_results


def parse_arguments():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Build NuttX with specified board configuration"
    )
    parser.add_argument("board", help="Board configuration to use")
    parser.add_argument(
        "--path", default="nuttx", help="Path to NuttX directory (default: nuttx)"
    )
    parser.add_argument("--json", help="JSON file to append build results to")
    parser.add_argument("--crate", help="Specific crate to build instead of all crates")
    parser.add_argument(
        "--run", action="store_true", help="Run the binary after building"
    )

    return parser.parse_args()


def collect_crates(parent_dir, specific_crate=None):
    """Collect crate directories and their config options."""
    collector = Collector(f"{parent_dir}/crates")
    print(f"📦 Found {len(collector.crate_dirs)} crate directories:")
    for crate_dir in collector.crate_dirs:
        print(f"  - {os.path.basename(crate_dir)}")

    # Get the main Kconfig option from the Kconfig file
    kconfig_option = collector.get_crate_config_mapping()

    # Filter crates if a specific one is requested
    if specific_crate:
        filtered_crates = {
            k: v
            for k, v in kconfig_option.items()
            if os.path.basename(k) == specific_crate
        }
        if not filtered_crates:
            print(f"❌ Error: Crate '{specific_crate}' not found")
            return None
        kconfig_option = filtered_crates
        print(f"🔍 Building only crate: {specific_crate}")

    return kconfig_option


def build_baseline(board_config, nuttx_path):
    """Build the baseline NuttX configuration."""
    builder = Builder(f"{board_config}:nsh", nuttx_path)
    print(f"🔨 Building NuttX Baseline")
    start_time = time.time()
    builder.configure()
    baseline = builder.build()
    baseline_build_time = time.time() - start_time
    print(
        f"📊 Baseline size: text={baseline['text']} data={baseline['data']} "
        f"bss={baseline['bss']} total={baseline['total']}, build time: {baseline_build_time:.2f}s"
    )
    return builder, baseline


def build_crates(
    kconfig_options, builder, baseline, json_manager, start_timestamp, args=None
):
    """Build each crate with its configuration option."""
    # Create a runner for executing the binary if needed
    runner = None
    if args and args.run:
        binary_path = f"{builder.build_dir}/nuttx"
        try:
            runner = Runner(binary_path, args.board)
        except Exception as e:
            # Warn if the runner cannot be created, it will be skipped
            print(f"ℹ️ Info: Unable to create runner, skip test: {str(e)}")

    for crate_path, config_option in kconfig_options.items():
        crate_name = os.path.basename(crate_path)
        print(f"\n🔧 Building crate: {crate_name} with option: {config_option}")

        start_time = time.time()
        builder.configure([("enable", config_option)])
        crate_size = builder.build()
        build_time = time.time() - start_time

        # Print build results and get size differences
        diffs = print_build_results(crate_name, baseline, crate_size, build_time)

        # Initialize test results
        test_time = None
        test_output = None
        test_success = None

        # Run the binary if requested
        if runner:
            binary_path = f"{builder.build_dir}/nuttx"
            if os.path.exists(binary_path):
                print(f"🚀 Running test for crate: {crate_name}")
                try:
                    print(f"⚙️ Executing command: rust_crate_test_{crate_name}")
                    execution_time, output, success = runner.run(
                        f"rust_crate_test_{crate_name}"
                    )

                    # Store test results
                    test_time = execution_time
                    test_output = output
                    test_success = success

                    print(f"⏱️ Command execution time: {execution_time:.2f} seconds")
                    if success:
                        print(f"✅ Success")
                    else:
                        print(f"❌ Failure")
                    print("📝 Output:")
                    print(output)
                except Exception as e:
                    print(f"❌ Error running binary: {str(e)}")
                    test_success = False
                    test_output = str(e)
            else:
                print(f"❌ Binary not found at: {binary_path}")
                test_success = False
                test_output = "Binary not found"

        # Always collect results, now including test results
        json_manager.append_result(
            crate_name,
            baseline,
            crate_size,
            diffs,
            start_timestamp,
            test_time,
            test_output,
            test_success,
        )

    # Stop the runner when done
    if runner:
        runner.stop()


def main():
    """
    Main entry point for the NuttX build script.

    Parses command line arguments and initiates the build process
    with the specified configuration.
    """
    # Capture timestamp at the beginning of execution
    start_timestamp = int(time.time())

    # Parse command line arguments
    args = parse_arguments()

    # Get parent dir from the script file location
    script_dir = os.path.dirname(os.path.abspath(__file__))
    parent_dir = os.path.dirname(script_dir)

    # Collect and filter crates
    kconfig_options = collect_crates(parent_dir, args.crate)
    if kconfig_options is None:  # Error in crate collection
        return

    # Build the baseline
    builder, baseline = build_baseline(args.board, args.path)

    # Create JSON result manager
    json_manager = JsonResultManager(args.json)

    # Build each crate
    build_crates(
        kconfig_options, builder, baseline, json_manager, start_timestamp, args
    )

    # Save results
    json_manager.flush()
    if args.json:
        print(f"💾 Results saved to {args.json}")

    # Calculate and display total execution time
    end_timestamp = time.time()
    execution_time = end_timestamp - start_timestamp
    print(f"⏱️ Total execution time: {execution_time:.2f} seconds")


if __name__ == "__main__":
    main()
