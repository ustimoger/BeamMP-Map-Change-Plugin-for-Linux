map_vote_count = {}

function ChatMessageHandler(sender_id, sender_name, message)
print(message)
if  string.find(message, "/vote") then 
    local startind, endind= string.find(message, "/vote")
          local cutoff = string.sub(message, endind, message.length)
 if string.find(cutoff, "start") then
               ExecCommand("trackselect")
               MP.CreateEventTimer("lockMap", 60000, nil)
               -- start timer and take map requests, after timer runs out, selct highest voted map and restart the server using ./BeamNGEdit restart

else
   
  
  map_vote_count[tonumber(cutoff)] = map_vote_count[tonumber(cutoff)] + 1

end
      

end 

end

function ExecCommand(commPass)
    local command = "./BeamNGEdit {r}"
        command  = command:gsub('{r}',commPass)

        MP.SendChatMessage(-1, os.capture(command, true))
end


function os.capture(cmd, raw)
    local f = assert(io.popen(cmd, 'r'))
    local s = assert(f:read('*a'))
    f:close()
    if raw then return s end
    s = string.gsub(s, '^%s+', '')
    s = string.gsub(s, '%s+$', '')
    s = string.gsub(s, '[\n\r]+', ' ')
    return s
  end

function lockMap(map_vote_count)

    MP.CancelEventTimer("lockMap")
local highest = 0
for i in map_vote_count.length do
if tonumber(map_vote_count[i]) > tonumber(map_vote_count[highest]) then 
    highest = i 
end 
end
local comm = "trackselect {n}"
local comm = comm:gsub('{n}', tostring(highest))
    ExecCommand(comm)
end



MP.RegisterEvent("lockMap", "lockMap")
MP.RegisterEvent("onChatMessage","ChatMessageHandler")