import subprocess
import json

CMD = "grpcurl -plaintext -import-path ./proto -proto auth.proto"

def test_login():
    result = subprocess.run(
        CMD.split(' ') + ["-d", '{"email": "test@mail.com", "password": "test"}', "app:50051", "auth.Auth/Login"],
        capture_output=True,
        text=True
    )

    response = json.loads(result.stdout)
    assert response["token"] == "token"

def test_register():
    result = subprocess.run(
        CMD.split(' ') + ["-d", '{"email": "test@mail.com", "password": "test"}', "app:50051", "auth.Auth/Register"],
        capture_output=True,
        text=True
    )

    response = json.loads(result.stdout)
    assert response["token"] == "token"