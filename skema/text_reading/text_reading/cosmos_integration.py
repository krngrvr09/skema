#!/usr/bin/env python3

# This script was inherited from the AutoMATES project.

"""
Script to process COSMOS' output in Parquet format into a JSON format suitable
for consumption by the SKEMA text reading pipeline.
"""

import pandas as pd
import json
import sys
import re
import os
import argparse
from pathlib import Path

from tqdm import tqdm


def main(parquet_file_folder: str, output_dir:str):

    parquet_files = os.listdir(parquet_file_folder)

    for filename in tqdm(parquet_files, desc="Converting parquets"):
        if filename.endswith(".parquet"):
            parquet_filepath = os.path.join(parquet_file_folder, filename)
            parquet_df = pd.read_parquet(parquet_filepath)
            parquet_json = parquet_df.to_json()
            parquet_data = json.loads(parquet_json)

            if len(parquet_data) > 0:
                parquet_data_keys = list(parquet_data.keys())
                num_data_rows = max(
                    [int(k) for k in parquet_data[parquet_data_keys[0]]]
                )

                row_order_parquet_data = [dict() for i in range(num_data_rows + 1)]
                for field_key, row_data in parquet_data.items():
                    for row_idx, datum in row_data.items():
                        row_idx_num = int(row_idx)
                        row_order_parquet_data[row_idx_num][field_key] = datum

                main_doc_re = r"documents_[a-zA-Z0-9]*\.parquet"
                if re.match(main_doc_re, filename) is not None:
                    # if filename == "documents.parquet":
                    # Sorts the content sections by page number and then by
                    # bounding box location. Use x-pos first to account for
                    # multi-column documents and then sort by y-pos.
                    row_order_parquet_data.sort(
                        key=lambda d: (
                            d["page_num"],
                            d["bounding_box"][0]
                            // 500,  # allows for indentation while still catching items across the center line
                            # (d["bounding_box"][0]) // 100
                            # + round((d["bounding_box"][0] % 100 // 10) / 10),
                            d["bounding_box"][1],
                        )
                    )

                    edits = list()
                    for e1, extraction1 in enumerate(row_order_parquet_data):
                        (ext1_x1, ext1_y1, ext1_x2, ext1_y2) = extraction1[
                            "bounding_box"
                        ]
                        # Don't bother processing for left-justified or centered
                        # content ... only right column content needs to be checked
                        if ext1_x1 < 500:
                            continue

                        ext1_page_num = extraction1["page_num"]
                        found_col_break = False
                        insertion_index = -1
                        t1 = e1
                        while t1 > 0:
                            extraction2 = row_order_parquet_data[t1 - 1]
                            ext2_page_num = extraction2["page_num"]
                            # If the previous sorted entry is on an earlier page
                            # then we can stop our search
                            if ext1_page_num > ext2_page_num:
                                break

                            (ext2_x1, ext2_y1, ext2_x2, ext2_y2) = extraction2[
                                "bounding_box"
                            ]

                            if ext1_y2 <= ext2_y1:
                                ext2_xspan = ext2_x2 - ext2_x1
                                # Useful heuristic cutoff for now
                                if ext2_xspan >= 800:
                                    found_col_break = True
                                    insertion_index = t1 - 1
                            t1 -= 1
                        if found_col_break:
                            edits.append(
                                {
                                    "del_idx": e1,
                                    "ins_idx": insertion_index,
                                    "val": extraction1,
                                }
                            )
                    for edit_dict in edits:
                        del row_order_parquet_data[edit_dict["del_idx"]]
                        row_order_parquet_data.insert(
                            edit_dict["ins_idx"], edit_dict["val"]
                        )
                    row_order_parquet_data.sort(key=lambda d: (d["pdf_name"]))

                    name2results = dict()
                    for row_data in row_order_parquet_data:
                        pdf_name = row_data["pdf_name"].replace(".pdf", "")
                        if row_data["pdf_name"] in name2results:
                            name2results[row_data["pdf_name"]].append(row_data)
                        else:
                            name2results[row_data["pdf_name"]] = [row_data]
                    for pdf_name, pdf_data in name2results.items():
                        no_end_pdf_name = pdf_name.replace(".pdf", "")
                        pdf_json_data_path = os.path.join(output_dir, Path(parquet_filepath.replace(
                            ".parquet",
                            f"{no_end_pdf_name}--COSMOS-data.json")).name,
                        )
                        json.dump(pdf_data, open(pdf_json_data_path, "w"))

                else:
                    parquet_json_filepath = os.path.join(output_dir, Path(parquet_filepath.replace(
                        ".parquet", ".json"
                    )).name)
                    json.dump(
                        row_order_parquet_data, open(parquet_json_filepath, "w")
                    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )

    parser.add_argument(
        "input",
        help="The input directory with the COSMOS Parquet files to process.",
    )

    parser.add_argument(
        "-o",
        "--output",
    )

    args = parser.parse_args()
    main(args.input, args.output)
