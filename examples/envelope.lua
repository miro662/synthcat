function envelope(attack, decay, key_time, release, sustain_level)
    return spline({
        {0.0, 0.0},
        {attack, 1.0},
        {attack + decay, sustain_level},
        {key_time, sustain_level},
        {key_time + release, 0.0}
    })
end

return wave(440) * envelope(0.01, 0.001, 0.2, 0.1, 0.5)