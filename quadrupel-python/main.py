# run `build_python_bindings.sh` to create this library
# noinspection PyUnresolvedReferences
from quadrupel import parse_message_from_drone, create_message_for_drone

if __name__ == '__main__':
    print(create_message_for_drone("""
    {
        "TargetYaw": 10
    }
    """))