
function ChatMessageHandler(sender_id, sender_name, message)
print(message)
if  string.find(message, "/vote") then 
    local startind, endind= string.find(message, "/vote")
          local cutoff = string.sub(message, endind, message.length)
          if string.find(cutoff, "start") then
               ExecCommand("trackselect")
               -- start timer and take map requests, after timer runs out, selct highest voted map and restart the server using ./BeamNGEdit restart
end 
      

end 

end


function ExecCommand(commPass)
    local command = "./BeamNGEdit {r}"
        command  = command:gsub('{r}',commPass)

        os.capture(command, true); -- add server out message 
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

MP.RegisterEvent("onChatMessage","ChatMessageHandler")