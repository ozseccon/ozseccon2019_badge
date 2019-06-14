import hid
import time
import argparse
import json

# enumerate USB devices

track1 = "%B555555555555555555^BILL/CLINTON^291110100000931?"
track2 = ";66666666666=29111010000093100000?"
track3 = ";56=87?"

track4 = "%B66666666666666^HILLARY/CLINTON^291110100000931?"
track5 = ";77777777=29111010000093100000?"
track6 = ";44=55?"

def chunks(l):
    """Yield successive 60-char-sized chunks from l."""
    #pad l to n+60 bytes long
    n = 60 
    padded_l = l + ((n - len(l) % n) * [0]) 
    for i in range(0, len(padded_l), n):
        yield padded_l[i:i + n]

def gen_track_data(track1, track2, track3):
    trackdata_text = chr(len(track1))+track1+chr(len(track2))+track2+chr(len(track3))+track3
    trackdata_bin = map(ord,trackdata_text)
    chunked_tracks = list(chunks(trackdata_bin))
    return chunked_tracks

def load_card(card1, card2, card3):
    #generate track data 
    #cmd structure is [header, data_1, data_2, &otherdata[61]
    #cmd 7 is to load a card.
    #data_1, track to load
    #data_2, chunk number   
    #other_data, track data to load 
    #load track 1
    #write1 = gen_track_data(track1,track2,track3) 
    #write2 = gen_track_data(track4,track5,track6)
 
    try: 
        h = hid.device()
        h.open(0xffff, 0xffff) # OzSecCon 

        print("Manufacturer: %s" % h.get_manufacturer_string())
        print("Product: %s" % h.get_product_string())
        print("Serial No: %s" % h.get_serial_number_string())

        # enable non-blocking mode
        h.set_nonblocking(1)
        h.write([0x07]+[0x00]*63) #erase flash
        
        for i, track_chunk in enumerate(card1):
            #print(track_chunk) 
            h.write([0x08]+[0x00]+[i]+track_chunk)
            #d = h.read(64) #check status 
       
        for i, track_chunk in enumerate(card2):
            #print(track_chunk) 
            h.write([0x08]+[0x01]+[i]+track_chunk)
            #d = h.read(64) #check status 
        
        for i, track_chunk in enumerate(card3):
            #print(track_chunk) 
            h.write([0x08]+[0x02]+[i]+track_chunk)
            #d = h.read(64) #check status 

        print("Closing the device")
        h.close()
    except IOError as ex:
        print(ex)
        print("You probably don't have the hard coded device. Update the hid.device line")
        print("in this script with one from the enumeration list output above and try again.")

def splash_logo():
    logo = """________             _________              _________                   
\_____  \  ________ /   _____/  ____   ____ \_   ___ \   ____    ____   
 /   |   \ \___   / \_____  \ _/ __ \_/ ___\/    \  \/  /  _ \  /    \  
/    |    \ /    /  /        \\  ___/\  \___\     \____(  <_> )|   |  \ 
\_______  //_____ \/_______  / \___  >\___  >\______  / \____/ |___|  / 
        \/       \/        \/      \/     \/        \/              \/  
                                                                        

________ _______   ____  ________  
\_____  \\   _  \ /_   |/   __   \ 
 /  ____//  /_\  \ |   |\____    / 
/       \\  \_/   \|   |   /    /  
\_______ \\_____  /|___|  /____/   
        \/      \/                 
                                   

_________                     .___.____                       .___               
\_   ___ \ _____  _______   __| _/|    |     ____ _____     __| _/ ____ _______  
/    \  \/ \__  \ \_  __ \ / __ | |    |    /  _ \\__  \   / __ |_/ __ \\_  __ \ 
\     \____ / __ \_|  | \// /_/ | |    |___(  <_> )/ __ \_/ /_/ |\  ___/ |  | \/ 
 \______  /(____  /|__|   \____ | |_______ \\____/(____  /\____ | \___  >|__|    
        \/      \/             \/         \/           \/      \/     \/         
usage: sudo python ./cardutil.py --cardfile [json formatted card file]"""
 
    return logo


#load_card()

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--nologo", help="disable logo print", action="store_false")
    parser.add_argument("--cardfile", help="load JSON formatted card data", type=argparse.FileType('r')) 
    #parser.add_argument("--track1", help="load track 1 data, ascii encoded")
    #parser.add_argument("--track2", help="load track 2 data, ascii encoded")
    #parser.add_argument("--track3", help="load track 3 data, ascii encoded")
    args = parser.parse_args()
    if args.nologo:
        print(splash_logo())
    if args.cardfile: 
        with args.cardfile as json_file: #filedata = args.cardfile.readlines()
            print("[*] loading cards from {}".format(json_file.name))
            data = json.load(json_file)
            print("[*] loading card 1:".format(data['card1']['track1']))
            print("[*] track1: {}".format(data['card1']['track1']))
            print("[*] track2: {}".format(data['card1']['track2']))
            print("[*] track3: {}".format(data['card1']['track3']))
            card1 = gen_track_data(data['card1']['track1'], data['card1']['track2'], data['card1']['track3'])
            print("[*] loading card 2:".format(data['card1']['track1']))
            print("[*] track1: {}".format(data['card2']['track1']))
            print("[*] track2: {}".format(data['card2']['track2']))
            print("[*] track3: {}".format(data['card2']['track3']))
            card2 = gen_track_data(data['card2']['track1'], data['card2']['track2'], data['card2']['track3'])
            print("[*] loading card 3:".format(data['card1']['track1']))
            print("[*] track1: {}".format(data['card3']['track1']))
            print("[*] track2: {}".format(data['card3']['track2']))
            print("[*] track3: {}".format(data['card3']['track3']))
            card3 = gen_track_data(data['card3']['track1'], data['card3']['track2'], data['card3']['track3'])
            load_card(card1, card2, card3) 
            print("[*] finished")
