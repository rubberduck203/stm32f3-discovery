#
# Cortex-Debug extension calls this function during initialization. You can copy this
# file, modify it and specifyy it as one of the config files supplied in launch.json
# preferably at the beginning.
#
# Note that this file simply defines a function for use later when it is time to configure
# for SWO.
#
set USE_SWO 0
proc CDSWOConfigure { CDCPUFreqHz CDSWOFreqHz CDSWOOutput } {
    # Alternative option: Pipe ITM output into itm.txt file
    # tpiu config internal itm.txt uart off $CDCPUFreqHz

    # Default option so SWO display of VS code works.
    tpiu config internal $CDSWOOutput uart off $CDCPUFreqHz $CDSWOFreqHz
    itm port 0 on
}
