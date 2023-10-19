
function ChatMessageHandler(sender_id, sender_name, message)
print(message)
if  string.find(message, "/vote") then 
    local startind, endind= string.find(message, "/vote")
          local cutoff = string.sub(message, endind, message.length)

      

end 

end


function ExecCommand(commPass)
    local command = "./BeamNGEdit {r}"
        command  = command:gsub('{r}',commPass)

        os.execute(command)
end



MP.RegisterEvent("onChatMessage","ChatMessageHandler")