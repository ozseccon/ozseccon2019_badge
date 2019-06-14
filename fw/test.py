from __future__ import print_function

import hid
import time

# enumerate USB devices

for d in hid.enumerate():
    keys = list(d.keys())
    keys.sort()
    for key in keys:
        print("%s : %s" % (key, d[key]))
    print()

try:
    print("opening the device")

    h = hid.device()
    h.open(0xffff, 0xffff) # OzSecCon 

    print("Manufacturer: %s" % h.get_manufacturer_string())
    print("Product: %s" % h.get_product_string())
    print("Serial No: %s" % h.get_serial_number_string())

    # enable non-blocking mode
    h.set_nonblocking(1)

    # write some data to the device
    while(1): 
        print("Write the data")
        #h.write([0, 63, 35, 35] + [0] * 61)
        #h.write([0x01,0x00,0x00,0x00,0x00,0x00])
        h.write([0x01]+[0x01]+[0x00]*63)

        # wait
        time.sleep(0.25)
        h.write([0x01]+[0x00]+[0x00]*63)
        h.write([0x02]+[0x01]+[0x00]*63)
        time.sleep(0.25)
        h.write([0x02]+[0x00]+[0x00]*63)
        h.write([0x03]+[0x01]+[0x00]*63)
        time.sleep(0.25)
        h.write([0x03]+[0x00]+[0x00]*63)
        h.write([0x04]+[0x01]+[0x00]*63)
        time.sleep(0.25)
        h.write([0x04]+[0x00]+[0x00]*63)
        h.write([0x05]+[0x01]+[0x00]*63)
        time.sleep(0.25)
        h.write([0x05]+[0x00]+[0x00]*63)
        h.write([0x06]+[0x01]+[0x00]*63)
        time.sleep(0.25)
        h.write([0x06]+[0x00]+[0x00]*63)
        #h.write([0x02,0x00,0x00,0x00,0x00,0x00])
        #h.write([0x03,0x00,0x00]+[0x42]*107)
    #h.write([0x03,0x00,0x00,0x00,0x00,0x00])
    # read back the answer
    #print("Read the data")
    #while True:
    #    d = h.read(64)
    #    if d:
    #        print(d)
    #    else:
    #        break

    print("Closing the device")
    h.close()

except IOError as ex:
    print(ex)
    print("You probably don't have the hard coded device. Update the hid.device line")
    print("in this script with one from the enumeration list output above and try again.")

print("Done")

