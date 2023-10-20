Map_vote_count = {}
Mscount = 0
Blacklisted_senders = {}
VoteCount = 1
Seconds = 0
Vote_has_started = false 
function ChatMessageHandler(sender_id, sender_name, message)
print(message)
if  string.find(message, "/vote ") then 
    local startind, endind= string.find(message, "/vote")
          local cutoff = string.sub(message, endind+2, message.length)
 if string.find(cutoff, "start") and not Vote_has_started  then
Vote_has_started = true 
               ExecCommand("trackselect", true)
               MP.CreateEventTimer("lockMap", 1000, MP.CallStrategy.BestEffort)
               -- start timer and take map requests, after timer runs out, selct highest voted map and restart the server using ./BeamNGEdit restart

else
   
  if tonumber(cutoff)~= nil and not has_value(Blacklisted_senders, sender_id) then 
  
    
    print("A vote has been registered")
  Map_vote_count[tonumber(cutoff)] = Map_vote_count[tonumber(cutoff)] + 1
  Blacklisted_senders[VoteCount] = sender_id
  VoteCount = VoteCount + 1
  else 
    if tonumber(cutoff)== nil then
      MP.SendChatMessage(sender_id, "Please enter a proper value" )
    end 
if has_value(Blacklisted_senders, sender_id) then 
  MP.SendChatMessage(sender_id, "Sorry you already voted" )
end 
  end 
end
      

end 

end

function ExecCommand(commPass, boo)
    local command = "*replacehere*/BeamNGEdit {r}"
   
        command  = command:gsub('{r}',commPass)
      if boo then Mscount = 0 end 
        for line in os.capture(command, true):gmatch("[^\r\n]+") do 
        MP.SendChatMessage(-1, line )
       if boo then  Mscount = Mscount+1 end 
        end
        if boo then
        for i = 0, Mscount -2 do 
            Map_vote_count[i] = 0
        end
        end 
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

if Seconds > 45 or Seconds == 30 then 
  MP.SendChatMessage(-1, "There is ".. 61 - Seconds.. " Seconds left to vote before Restart." )

end 

  if Seconds > 60 then 
    MP.CancelEventTimer("lockMap")
local highest = 0
for i = 0, Mscount -2 do
if tonumber(Map_vote_count[i]) > tonumber(Map_vote_count[highest]) then 
    highest = i 
end 
end
local comm = "trackselect {n}"
local comm = comm:gsub('{n}', tostring(highest))
    ExecCommand(comm, false)
end 

Seconds = Seconds +1
end



MP.RegisterEvent("lockMap", "lockMap")
MP.RegisterEvent("onChatMessage","ChatMessageHandler")

 function has_value (tab, val)
  for index, value in ipairs(tab) do
      if value == val then
          return true
      end
  end

  return false
end