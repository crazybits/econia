from os import environ

import paho.mqtt.client as mqtt
import json

# This is an example on how to use subscribe to events emitted by the DSS.

# Once you have a DSS running (see
# https://econia.dev/off-chain/dss/data-service-stack on how to do that), you
# can run this script by running:
#
# $ poetry install
# $ poetry run event
#
# It will subscribe to all fill events and to place limit order events on
# market 7.
#
# Each MQTT message has a topic and a payload. You can subscribe to topics.
# When specifying the topic you want to subscribe to, you can put a + instead
# of a value to subscribe to all events matching the rest of the topic.
#
# Example:
#
# We use the topic fill/MARKET_ID/USER_ADDRESS/CUSTODIAN_ID. Fill events by the
# user 0xdeadbeef with custodian ID 1 on market 3 will have fill/3/0xdeadbeef/1
# as a topic. But you could subscribe to fill/+/0xdeadbeef/+ to get all fill
# events from the user 0xdeadbeef.

HOST = "127.0.0.1"
PORT = 21883

def get_host() -> str:
    host = environ.get("MQTT_HOST")
    if host == None:
        host_in = input(
            "Enter the MQTT host (enter nothing to use 127.0.0.1).\n"
        ).strip()
        if host_in == "":
            return HOST
        else:
            return host_in
    else:
        return host

def get_port() -> int:
    port = environ.get("MQTT_PORT")
    if port == None:
        port_in = input(
            "Enter the MQTT port (enter nothing to use 21883 which is the default MQTT port when depolying the DSS with docker compose).\n"
        ).strip()
        if port_in == "":
            return PORT
        else:
            return int(port_in)
    else:
        return int(port)

def get_topic() -> str:
    topic = environ.get("MQTT_TOPIC")
    if topic == None:
        event_type_in = input(
            "Enter the event type to subscribe to (enter + or nothing to subscribe to all event types, or fill, place_limit_order, cancel_order, etc.).\n"
        ).strip()
        if event_type_in == "":
            event_type_in = "+"
        market_id_in = input(
            "Enter the market ID of the market you want to watch (enter + or nothing to subscribe to events from all markets).\n"
        ).strip()
        if market_id_in == "":
            market_id_in = "+"
        return "{}/{}/#".format(event_type_in, market_id_in)
    else:
        return topic

def on_connect(client, userdata, flags, reason_code, properties):
    # Subscribe to the given topic
    topic = get_topic()
    print("Subscribing to {}.".format(topic))
    client.subscribe(topic)

def on_message(client, userdata, msg):
    # Decode the JSON payload.
    data = json.loads(msg.payload.decode("utf-8"))

    # Print data about the event.
    print("New event with topic " + msg.topic + " on market " + str(data["market_id"]) + ".")

# Create a new mqtt client
mqttc = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, transport='websockets')

# Set the handling functions
mqttc.on_connect = on_connect
mqttc.on_message = on_message

# Connect to the MQTT server (mosquitto)
mqttc.connect(get_host(), get_port(), 60)

mqttc.loop_forever()
