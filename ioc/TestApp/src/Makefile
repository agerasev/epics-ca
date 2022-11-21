TOP=../..

include $(TOP)/configure/CONFIG
#----------------------------------------
#  ADD MACRO DEFINITIONS BELOW HERE

# Use typed rset structure (see 3.16.1 release notes)
USR_CPPFLAGS += -DUSE_TYPED_RSET

# Build the IOC application
PROD_IOC = Test

# Test.dbd will be created and installed
DBD += Test.dbd

# Test.dbd will include these files:
Test_DBD += base.dbd

# Test_registerRecordDeviceDriver.cpp derives from Test.dbd
Test_SRCS += Test_registerRecordDeviceDriver.cpp

# Build the main IOC entry point
Test_SRCS += TestMain.cpp

# Link QSRV (pvAccess Server) if available
ifdef EPICS_QSRV_MAJOR_VERSION
    Test_LIBS += qsrv
    Test_LIBS += $(EPICS_BASE_PVA_CORE_LIBS)
    Test_DBD += PVAServerRegister.dbd
    Test_DBD += qsrv.dbd
endif

# Finally link IOC to the EPICS Base libraries
Test_LIBS += $(EPICS_BASE_IOC_LIBS)

include $(TOP)/configure/RULES
#----------------------------------------
#  ADD EXTRA GNUMAKE RULES BELOW HERE
