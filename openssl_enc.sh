#!/bin/bash

openssl enc -des-ecb -in file.iso -out file.openssl.iso.des -nosalt -nopad -K 7275737472757374
