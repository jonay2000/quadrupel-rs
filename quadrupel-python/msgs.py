
def abort() -> str:
    return change_state("Panic")


def change_state(state: str) -> str:
    return f"""
    {{
        "ChangeState": "{state}" 
    }}
    """

def heartbeat() -> str:
    return f"""{{ "HeartBeat": 1 }} """

def motor_message(motor: int, value: int) -> str:
    return f"""
    {{
        "MotorValue": {{
            "motor": "M{motor}",
            "value": {value} 
        }}
    }}
    """
