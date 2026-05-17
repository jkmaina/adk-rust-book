#!/usr/bin/env python3
from __future__ import annotations

import sys
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MANIFESTS = sorted(ROOT.glob("chapter*/**/Cargo.toml"))


def parse_version(value: str) -> tuple[int, int]:
    major, minor, *_rest = value.split(".")
    return int(major), int(minor)


def is_at_least_1_92(value: str) -> bool:
    try:
        return parse_version(value) >= (1, 92)
    except Exception:
        return False


def main() -> int:
    issues: list[str] = []

    for manifest in MANIFESTS:
        rel = manifest.relative_to(ROOT)
        text = manifest.read_text()
        data = tomllib.loads(text)
        package = data.get("package", {})
        edition = str(package.get("edition", ""))
        rust_version = str(package.get("rust-version", ""))

        if edition != "2024":
            issues.append(f"{rel}: edition is {edition or '<missing>'}, expected 2024")

        if not rust_version or not is_at_least_1_92(rust_version):
            issues.append(
                f"{rel}: rust-version is {rust_version or '<missing>'}, expected 1.92+"
            )

        if '0.1.2' in text:
            issues.append(f"{rel}: contains legacy 0.1.2 dependency pin")

        dependencies = data.get("dependencies", {})
        for name, spec in dependencies.items():
            if not (name == "adk-rust" or name.startswith("adk-")):
                continue

            if isinstance(spec, str):
                issues.append(
                    f"{rel}: dependency {name} uses version string {spec!r} instead of workspace/path wiring"
                )
                continue

            if not isinstance(spec, dict):
                continue

            if spec.get("workspace") is True:
                continue

            path_value = spec.get("path")
            if isinstance(path_value, str) and "../adk-rust/" in path_value:
                continue

            version_value = spec.get("version")
            if version_value is not None:
                issues.append(
                    f"{rel}: dependency {name} pins version {version_value!r} instead of workspace/path wiring"
                )

    if issues:
        print("Edition/dependency drift detected:")
        for issue in issues:
            print(f"  - {issue}")
        return 1

    print(f"Checked {len(MANIFESTS)} manifests: edition/rust-version/dependency drift clean.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
