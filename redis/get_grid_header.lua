local key = redis.pcall('hget', 'key_hashes', ARGV[1])

if not key then
    return false
end

if key == '/' then
    local name_key = string.gsub('/:name:$lang', '%$(%w+)', {lang=ARGV[2]})
    return redis.pcall('get', name_key)
else
    local segments = {}
    for segment in string.gmatch(key, '([^/]+)') do
        table.insert(segments, segment)
    end

    local name_keys = {}
    for i, segment in pairs(segments) do
        name_keys[i] = string.gsub('$segment:name:$lang', '%$(%w+)', {segment=segment, lang=ARGV[2]})
    end

    local names = redis.pcall('mget', unpack(name_keys))

    local header = ''
    for _, name in pairs(names) do
        header = header .. ' / ' .. name
    end

    return string.gsub(header, '^%s*(.-)%s*$', '%1')
end
