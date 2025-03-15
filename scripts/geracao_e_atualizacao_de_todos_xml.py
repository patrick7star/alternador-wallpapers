#!/usr/bin/python3 -BOO
"""
   Script executa um script que gera todos XML dos diretórios com imagens no
 diretórios com slidesshows.
"""

from os import (system as ExecutaSimplesComando)

SHEBANGS = "/usr/bin/python3 -BOO"
SCRIPT = "lib/slide_background/atualiza_configuracao_xml.py"
CMD = "{} {}".format(SHEBANGS, SCRIPT)

print("Comando: '%s'" % CMD)
ExecutaSimplesComando(CMD)
