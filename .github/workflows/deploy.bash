#!/usr/bin/env bash

ssh -oStrictHostKeyChecking=accept-new connor@cwfitz.com <<ENDFILE
docker pull cwfitzgerald/bve-reborn-site:latest
docker stop bve-reborn-site
docker rm bve-reborn-site
docker run -d --name bve-reborn-site --restart always -p 9004:8000 cwfitzgerald/bve-reborn-site:latest
ENDFILE
