local user = ARGV[1]

local message_id = redis.pcall('hget', 'feedback:pending', user)

if message_id then
    redis.pcall('hdel', 'feedback:pending', user)
    return message_id
end
