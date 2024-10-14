function envelope(attack, decay, key_time, release, sustain_level)
    return spline({
        {0.0, 0.0},
        {attack, 1.0},
        {attack + decay, sustain_level},
        {key_time, sustain_level},
        {key_time + release, 0.0}
    })
end

key_envelope = envelope(0.01, 0.001, 0.2, 0.1, 0.5)

function key(name)
    frequencies = {
        D4 = 293.66,
        E4 = 329.63,
        F4 = 349.23,
        G4 = 392.0,
        A4 = 440.0
    }
    return wave(frequencies[name]) * key_envelope
end

-- 3-4
melody = key("A4")
    + shift(key("F4"), 0.5)
    + shift(key("F4"), 1.0)
    ---
    + shift(key("G4"), 1.5)
    + shift(key("E4"), 2.0)
    + shift(key("E4"), 2.5)
    ---
    + shift(key("D4"), 3.0)
    + shift(key("F4"), 3.25)
    + shift(key("A4"), 3.5)
    ---
    + shift(key("A4"), 4.0)
    + shift(key("F4"), 4.5)
    + shift(key("F4"), 5.0)
    ---
    + shift(key("G4"), 5.5)
    + shift(key("E4"), 6.0)
    + shift(key("E4"), 6.5)
    ---
    + shift(key("D4"), 7.0)
    + shift(key("F4"), 7.25)
    + shift(key("D4"), 7.5)
    ---
    
return melody * 0.8