using DiscordRPC;
using System;

namespace ForceConnect.Utility
{
    public class DiscordRPCManager
    {
        private static DiscordRPCManager instance;
        private static RichPresence _lastPresecnce;
        private DiscordRpcClient client;

        private DiscordRPCManager()
        {
            // Initialize your Discord RPC client here
            client = new DiscordRpcClient("1148031843178917948");
            client.Initialize();

            if (_lastPresecnce != null)
            {
                client.SetPresence(_lastPresecnce);
                return;
            }
            // Set up your Discord RPC presence details here
            var presence = new RichPresence
            {
                Details = "Ideal",
                State = "",
                Assets = new Assets
                {
                    LargeImageKey = "force",
                    LargeImageText = "Powerful DnsChanger",
                    SmallImageKey = "small_image_key",
                    SmallImageText = "Small Image Text"
                },
                Buttons = new Button[]
                {
                    new Button() { Label = "Try it" , Url = "https://fc.mxqius.ir"}
                }
            };
            client.SetPresence(presence);
        }

        public static DiscordRPCManager GetInstance()
        {
            if (instance == null)
            {
                instance = new DiscordRPCManager();
            }

            return instance;
        }

        // Add methods for updating presence, handling events, etc. as needed

        public void UpdatePresence(string details = "ForceConnect", string state = "Ideal", string smallImage = "None", string smallImageText = "None", string largeImage = "forceconnect", string largeImageText = "Powerful DnsChanger")
        {
            var presence = new RichPresence
            {
                Details = details,
                State = state,
                Assets = new Assets
                {
                    LargeImageKey = largeImage,
                    LargeImageText = largeImageText,
                    SmallImageKey = smallImage,
                    SmallImageText = smallImageText
                },
                Buttons = new Button[]
                {
                    new Button() { Label = "Try it" , Url = "https://fc.mxqius.ir"}
                }
            };

            client.SetPresence(presence);
            _lastPresecnce = presence;
        }

        public void Dispose()
        {
            client.Dispose();
            instance = null;
        }
    }

}
