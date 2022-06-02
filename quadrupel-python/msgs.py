
def abort() -> str:
    return change_state("Panic")


def change_state(state: str) -> str:
    return f"""
    {{
        "ChangeState": "{state}" 
    }}
    """

def toggle_height_control() -> str:
    return f"""
    {{
        "SetHeightMode": 1 
    }}
    """

def toggle_raw_mode() -> str:
    return f"""
    {{
        "SetRawMode": 1 
    }}
    """

def heartbeat() -> str:
    return f"""
    {{
        "HeartBeat": 1
    }}
    """

def auto_land() -> str:
    return "\"AutoLand\""

def motor_message(motor: int, value: int) -> str:
    return f"""
    {{
        "MotorValue": {{
            "motor": "M{motor}",
            "value": {value} 
        }}
    }}
    """


def joystick_message(yaw: int, pitch: int, roll: int, lift: int) -> str:
    return f"""
    {{
        "TargetAttitude": {{
            "yaw": {yaw},
            "pitch": {pitch},
            "roll": {roll},
            "lift": {lift}
        }}
    }}
    """
