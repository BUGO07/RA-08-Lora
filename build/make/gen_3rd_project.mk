comma:=,
semi:=;
FILE_CREATE = $(file >$(1),$(2))
FILE_APPEND = $(file >>$(1),$(2))

PATH_PREFIX := $(shell realpath $(TREMO_SDK_PATH) --relative-to=.)/
SRC_PATH := $(foreach src, $($(PROJECT)_SOURCE),'$(addprefix $(PATH_PREFIX),$(shell realpath $(src) --relative-to=$(TREMO_SDK_PATH)))'$(comma))
LIB_PATH := $(foreach lib, $($(PROJECT)_LIBS),'$(addprefix $(PATH_PREFIX),$(shell realpath $(lib) --relative-to=$(TREMO_SDK_PATH)))'$(comma))
INC_PATH := $(foreach inc, $($(PROJECT)_INC_PATH),$(addprefix $(PATH_PREFIX),$(shell realpath $(inc) --relative-to=$(TREMO_SDK_PATH)))$(semi))
DEFINES := $(foreach dflags, $($(PROJECT)_DEFINES), $(dflags)$(comma))
ASMDEFINES := $(foreach adflags,$($(PROJECT)_AFLAGS),$(adflags)$(comma))
C_MiscControls := $($(PROJECT)_CFLAGS)
A_MissControls := $(AMISC)

ifeq ($($(PROJECT)_PRO_OUT),LIB)
OUTPUT_TARGET := $(subst lib,,$($(PROJECT)_LIB))
else
OUTPUT_TARGET := project
endif
