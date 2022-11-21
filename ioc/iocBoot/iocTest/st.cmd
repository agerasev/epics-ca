#!../../bin/linux-x86_64/Test

#- You may have to change Test to something else
#- everywhere it appears in this file

< envPaths

cd "${TOP}"

## Register all support components
dbLoadDatabase "dbd/Test.dbd"
Test_registerRecordDeviceDriver pdbbase

## Load record instances
dbLoadRecords "db/Test.db"

cd "${TOP}/iocBoot/${IOC}"
iocInit
