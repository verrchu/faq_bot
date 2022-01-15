local key = ARGV[1]
local user = ARGV[2]

local call_key = string.gsub('{l10n:none}:$key:likes', '%$(%w+)', {key=key})
local is_member = redis.pcall('sismember', call_key, user)

if is_member == 1 then
    redis.pcall('srem', call_key, user)

    return false
else
    redis.pcall('sadd', call_key, user)

    return true
end
