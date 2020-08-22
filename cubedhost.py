import http.client
import configparser


class CubedHostClient:
    CUBED_HOST_HOST = "prisma.cubedhost.com/api"

    def __init__(self):
        config = configparser.ConfigParser()
        config.read("project.properties")
        self.server_id = config.get('CubedHost', 'ServerId')
        self.token = config.get('CubedHost', 'ApiKey')
        self.user = config.get('CubedHost', 'ApiUser')

    def conn(self) -> http.client.HTTPSConnection:
        return http.client.HTTPSConnection(self.CUBED_HOST_HOST)

    def headers(self) -> dict:
        return {
            'X-Api-Key': self.token,
            'X-Api-User': self.user
        }

    def get_server_status(self):
        print(self.conn().request(
            "GET",
            f"/server/{self.server_id}",
            headers=self.headers()
        ))
