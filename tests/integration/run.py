#!/usr/bin/env python3

import sys
import traceback

tests = [
    ("node startup", "test_node_startup"),
    ("user discovery", "test_user_discovery"),
]

passed = 0
failed = 0

for name, mod_name in tests:
    print(f"\n[{name}]")
    try:
        mod = __import__(mod_name)
        mod.setup()
        try:
            for fn_name in dir(mod):
                if fn_name.startswith("test_"):
                    print(f" running {fn_name}...")
                    getattr(mod, fn_name)()
            passed += 1
        finally:
            mod.teardown()
    except AssertionError as e:
        print(f" FAIL: {e}")
        failed += 1
    except Exception as e:
        print(f" ERROR: {e}")
        traceback.print_exc()
        failed += 1


print(f"\n{'=' * 40}")
print(f"Results: {passed} passed, {failed} failed")
sys.exit(0 if failed == 0 else 1)
