local user = ARGV[1]

local message_id = redis.pcall('hget', 'feedback', user)

if message_id then
    redis.pcall('hdel', 'feedback', user)
    return message_id
end
