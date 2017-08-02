#
# Copyright 2016, NICTA
#
# This software may be distributed and modified according to the terms of
# the BSD 2-Clause license. Note that NO WARRANTY is provided.
# See "LICENSE_BSD2.txt" for details.
#
# @TAG(NICTA_BSD)
#

TARGETS := $(notdir ${SOURCE_DIR}).cdl
ADL := hellorust.camkes

main_object_HFILES := \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/main/include/*.h)) \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/include/*.h))
main_object_RUST := main
main_object_CFILES := \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/main/src/*.c)) \
		$(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/main/crt/*.c))

include ${PWD}/tools/camkes/camkes.mk
