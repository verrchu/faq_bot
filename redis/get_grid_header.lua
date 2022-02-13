local key = ARGV[1]
local lang = ARGV[2]

if key == '/' then
    local name_key = string.gsub('{l10n:$lang}:/:name', '%$(%w+)', {lang=lang})
    return redis.pcall('get', name_key)
else
    local segments = {}
    for segment in string.gmatch(key, '([^/]+)') do
        table.insert(segments, segment)
    end

    local name_keys = {}
    for i, segment in pairs(segments) do
        name_keys[i] = string.gsub('{l10n:$lang}:$segment:name', '%$(%w+)', {segment=segment, lang=lang})
    end

    local names = redis.pcall('mget', unpack(name_keys))

    local header = ''
    for _, name in pairs(names) do
        header = header .. ' / ' .. name
    end

    local header = string.gsub(header, '^%s*(.-)%s*$', '%1')
    local header = string.gsub(header, '([-*~`>#+=|.!(){}%[%]])', '\\%1')

    return header
end
