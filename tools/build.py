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
import json
import time
from builder import Builder
from collect import Collector


class JsonResultManager:
    """
    Class to manage reading and writing build results to JSON files.

    This class handles the persistence of build results, including baseline and
    crate-specific builds, allowing for tracking size differences across builds.
    It provides methods to append new build results and write the accumulated
    data to a JSON file for later analysis. If no JSON file is specified,
    the manager will collect data in memory but will not persist it.
    """

    def __init__(self, json_file):
        """
        Initialize the JsonResultManager with a file path and parse the JSON content.

        Args:
            json_file: Path to the JSON file to read from and write to, or None to collect data without saving
        """
        self.json_file = json_file
        self.data = self._parse_json()

    def _parse_json(self):
        """
        Parse the JSON file into a Python dictionary structure.

        Returns:
            dict: A dictionary containing all build results, with at least a "builds" list.
                 Returns {"builds": []} if the file doesn't exist or contains invalid JSON.
        """
        if not self.json_file or not os.path.exists(self.json_file):
            return {"builds": []}

        with open(self.json_file, "r") as f:
            try:
                return json.load(f)
            except json.JSONDecodeError:
                return {"builds": []}

    def append_result(
        self, board_config, crate_name, baseline, crate_size, diffs, timestamp
    ):
        """
        Append build results to the JSON data structure.

        This method adds a new build result entry to the data structure with information
        about the board configuration, crate name, baseline and crate build sizes,
        the differences between them, and a timestamp for when the build was performed.

        Args:
            board_config (str): Current board configuration identifier
            crate_name (str): Name of the crate being built
            baseline (dict): Baseline build sizes containing text, data, bss, and total
            crate_size (dict): Current crate build sizes containing text, data, bss, and total
            diffs (dict): Size differences between baseline and crate build
            timestamp (int): Unix timestamp for the build session
        """
        # Add new build result
        build_data = {
            "board_config": board_config,
            "crate_name": crate_name,
            "baseline": baseline,
            "crate_build": crate_size,
            "differences": {
                "text": diffs["text"],
                "data": diffs["data"],
                "bss": diffs["bss"],
                "total": diffs["total"],
            },
            "timestamp": timestamp,
        }

        self.data["builds"].append(build_data)

    def flush(self):
        """
        Write the current data back to the JSON file.

        This method serializes the internal data structure to JSON format
        and writes it to the file specified during initialization.
        If no file was specified, the data is discarded.
        """
        if not self.json_file:
            # No file specified, discard data
            return

        with open(self.json_file, "w") as f:
            json.dump(self.data, f, indent=2)

    def get_results(self):
        """
        Get the parsed JSON data structure.

        Returns:
            dict: A dictionary containing all build results, with at least a "builds" list
                 holding all the build data entries.
        """
        return self.data


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
    parser.add_argument("board_config", help="Board configuration to use")
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

    builder = Builder(args.board_config, args.path)
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

        # Calculate size differences
        text_diff = crate_size["text"] - baseline["text"]
        data_diff = crate_size["data"] - baseline["data"]
        bss_diff = crate_size["bss"] - baseline["bss"]
        total_diff = crate_size["total"] - baseline["total"]

        # Calculate percentage changes
        text_pct = (text_diff / baseline["text"] * 100) if baseline["text"] else 0
        data_pct = (data_diff / baseline["data"] * 100) if baseline["data"] else 0
        bss_pct = (bss_diff / baseline["bss"] * 100) if baseline["bss"] else 0
        total_pct = (total_diff / baseline["total"] * 100) if baseline["total"] else 0

        # Determine impact indicators based on percentage thresholds
        def get_impact_icon(pct):
            if pct > 50.0:
                return "🔴"  # Major increase (red)
            elif pct > 15.0:
                return "🟠"  # Moderate increase (orange)
            elif pct > 0.0:
                return "🟡"  # Minor increase (yellow)
            elif pct < 0.0:
                return "🟢"  # Decrease (green)
            else:
                return "⚪"  # No change (white)

        # Helper function to format differences with consistent spacing
        def format_diff(diff_val, pct_val):
            # Format the difference value with commas and sign
            diff_str = f"{diff_val:+,d}"
            # Format the percentage with a fixed width of 6 characters (including the sign and decimal point)
            pct_str = f"{pct_val:+6.1f}%"
            # Combine them with proper spacing to ensure alignment
            return f"{diff_str:>10} ({pct_str:>8})"

        # Format differences with consistent spacing
        text_str = format_diff(text_diff, text_pct)
        data_str = format_diff(data_diff, data_pct)
        bss_str = format_diff(bss_diff, bss_pct)
        total_str = format_diff(total_diff, total_pct)

        print(f"📊 Build Results for: {crate_name}")
        print("-" * 90)

        # Column headers with right-aligned difference and impact columns
        headers = ["Section", "Baseline", "With Crate", "Difference", "Impact"]
        print(
            f"{headers[0]:<10} {headers[1]:<15} {headers[2]:<15} {headers[3]:>25} {headers[4]:>10}"
        )
        print("-" * 90)

        # Table rows with consistent formatting and right-aligned difference and impact columns
        print(
            f"{'text':<10} {baseline['text']:,d} bytes{' '*(15-len(str(baseline['text']))-6)} "
            f"{crate_size['text']:,d} bytes{' '*(15-len(str(crate_size['text']))-6)} "
            f"{text_str:>25} {get_impact_icon(text_pct):^10}"
        )
        print(
            f"{'data':<10} {baseline['data']:,d} bytes{' '*(15-len(str(baseline['data']))-6)} "
            f"{crate_size['data']:,d} bytes{' '*(15-len(str(crate_size['data']))-6)} "
            f"{data_str:>25} {get_impact_icon(data_pct):^10}"
        )
        print(
            f"{'bss':<10} {baseline['bss']:,d} bytes{' '*(15-len(str(baseline['bss']))-6)} "
            f"{crate_size['bss']:,d} bytes{' '*(15-len(str(crate_size['bss']))-6)} "
            f"{bss_str:>25} {get_impact_icon(bss_pct):^10}"
        )
        print(
            f"{'total':<10} {baseline['total']:,d} bytes{' '*(15-len(str(baseline['total']))-6)} "
            f"{crate_size['total']:,d} bytes{' '*(15-len(str(crate_size['total']))-6)} "
            f"{total_str:>25} {get_impact_icon(total_pct):^10}"
        )
        print("-" * 90)
        print(f"⏱️  Build time: {build_time:.2f} seconds")

        # Always collect results, but only mention file writing when args.json is provided
        diffs = {
            "text": text_diff,
            "data": data_diff,
            "bss": bss_diff,
            "total": total_diff,
        }
        json_manager.append_result(
            args.board_config,
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
