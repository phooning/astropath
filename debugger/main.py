from flask import Flask
from flask_socketio import SocketIO, emit


app = Flask(__name__)
socketio = SocketIO(app)


def handle_message(data):
    print(f"Received message: " + str(data['type']))
    emit('receive_message', data, broadcast=True)


if __name__ == "__main__":
    socketio.run(app, host='0.0.0.0', port=5000)
