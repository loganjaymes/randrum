# DOCUMENTATION AND INFO
## MIDI AND `.mid`
### MIDI FILE STRUCTURE
- split into chunks of [ LABEL | SIZE | DATA ]
- entire midi file is list of chunks
- first chunk is header (bytes): 
    - 4: label (MThd)
    - 4: size (big endian)
    - 6: data of header
        - 2: format of file (0, 1, 2)
            - 0: header chunk w/ single track 
            - 1: header chunk w/ multiple tracks
            - 2 (RARE): header chunk w/ multiple patterns & one track each
        - 2: how many tracks in midi file
        - 2: number of divisons 
- rest of chunks:
    - MTrk
    - size
    - 8 bytes: events[]
        - we don't know how many events are in the array, so we must read them one by one
        - events begin w/ DeltaTime (amount of time since previous event) then EventStatus (type of event that occurred)
            - ex. Event = 90:
                - 9 = note press (8 is note off)
                - 0 = midi channel at index 0
            - if note on, next byte is which note is pressed. (see note vs octave table, range is [0-127]), then velocity of note 
- variable length quantities are used if we want a midi note longer than 255 ticks

### RUNNING STATUS
- if we omit a status byte in an event (i assume its to save memory, however i could be wrong), ie.

```
            E1: [00 90 3C 7F]
            E2: [00 XX 40 6E]
            E3: [18 XX 43 64]
```

- we can determine if the next byte is a status byte or data byte by checking the leftmost bit of the byte in question.
- fe., 40 = 0100 0000 => 0? => data byte 
- in this case, we must use the most recent status byte- that is, 90.

### BASIC MIDI EVENTS
| STATUS BYTE (EVENT, CHANNEL) |   DATA BYTES   |       DESCRIPTION       |
|:----------------------------:|:--------------:|:-----------------------:|
|              8?              | NOTE, VELOCITY |         NOTE OFF        |
|              9?              | NOTE, VELOCITY |         NOTE ON         |
|              A?              | NOTE, PRESSURE | POLYPHONIC KEY PRESSURE |
|              B?              | CONTROL, VALUE |      CONTROL CHANGE     |
|              C?              |     PROGRAM    |      PROGRAM CHANGE     |
|              D?              |    PRESSURE    |     CHANNEL PRESSURE    |
|              E?              |    LSB, MSB    |    PITCH WHEEL CHANGE   |

(credit to nobscode on youtube for the above table that i converted into markdown)

#### PROGRAM
- defined by general MIDI
- potentially important for merging tracks to ensure that notes are not written on the same channel, however this may be handled by the DAW when we export .mid files to be used and parsed.
- see [general midi for percussion](https://upload.wikimedia.org/wikipedia/commons/thumb/4/4e/GM_Standard_Drum_Map_on_vertical_keyboard.svg/960px-GM_Standard_Drum_Map_on_vertical_keyboard.svg.png)

#### META EVENTS
- 2 types (text, not text)
- **TEXT TYPE**: provide metadata such as copyright notice, sequence/track names, instrument names, etc.
    - not super important within this program's context
- **NON-TEXT TYPE**: sequence number, end of track, tempo, etc
    - ~again, not super important since the exported `.mid` will (at first) be imported to an actual DAW.~
    - ~in the future, these might be important if integrating a midi parser and soundfont to play the exported midi in a program.~
    - ***IMPORTANT***: `Set Tempo`, since every drum track may not be in 4/4
        - this is a feature that will be figured out..... later. for now i intend to focus on 4/4 and, in the forseeable future, 6/8, since they are the most two common time signatures.

## IMPORTANT REFERENCES
- [no bs code video](https://www.youtube.com/watch?v=P27ml4M3V7A)
- [javidx9 video](https://www.youtube.com/watch?v=040BKtnDdg0)
