# Copyright (c) 2025 Xiaomi Corporation
# SPDX-License-Identifier: Apache-2.0

"""
Utility functions for NuttX build tools.
"""


def print_build_results(crate_name, baseline, crate_size, build_time):
    """
    Print formatted build results comparing baseline with crate build.

    Args:
        crate_name: Name of the crate being built
        baseline: Dictionary containing baseline size metrics
        crate_size: Dictionary containing size metrics with the crate enabled
        build_time: Time taken to build the crate in seconds
    """
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

    # Return differences for use in result manager
    return {
        "text": text_diff,
        "data": data_diff,
        "bss": bss_diff,
        "total": total_diff,
    }
