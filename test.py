import paho.mqtt.client as mqtt

def create_client(id):
    print("Connecting")

    client = mqtt.Client(id)
    client.connect("localhost", port=1883)
    client.loop_start()

    return client

def close_client(client):
	client.disconnect()
	client.loop_stop()

def main():
    topic1 = "UPB/RPi_1"
    topic2 = "DORINEL/Zeus"
    topic3 = "TEST/arch"

    id = "mihnea"

    cl = create_client(id)

    payload1 = "{\"BAT\":99,\"HUMID\":40,\"PRJ\":\"SPRC\",\"TMP\":25.3}"
    payload2 = "{\"Alarm\":0,\"AQI\":12,\"BAT\":100,\"RSSI\":1500}"
    payload3 = "{\"BAT\":52}"

    cl.publish(topic1, payload1, qos=1)
    cl.publish(topic2, payload2, qos=1)
    cl.publish(topic3, payload3, qos=1)

    print('Done')

    close_client(cl)

if __name__ == "__main__":
	main()
