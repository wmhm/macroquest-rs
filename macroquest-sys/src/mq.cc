#include "macroquest-sys/include/mq.h"
#include "macroquest-sys/src/lib.rs.h"

namespace mqrust
{
    namespace mq
    {
        // Path Functions
        rust::Str get_path_MQRoot() { return ::mq::gPathMQRoot; }
        rust::Str get_path_Config() { return ::mq::gPathConfig; }
        rust::Str get_path_MQini() { return ::mq::gPathMQini; }
        rust::Str get_path_Macros() { return ::mq::gPathMacros; }
        rust::Str get_path_Logs() { return ::mq::gPathLogs; }
        rust::Str get_path_CrashDumps() { return ::mq::gPathCrashDumps; }
        rust::Str get_path_Plugins() { return ::mq::gPathPlugins; }
        rust::Str get_path_Resources() { return ::mq::gPathResources; }
        rust::Str get_path_EverQuest() { return ::mq::gPathEverQuest; }

        // General Functions
        void write_chat_color(rust::Str line, int color)
        {
            ::mq::WriteChatColor(static_cast<std::string>(line).c_str(), color);
        }

        // MQPlugin Functions
        rust::Str MQPlugin::plugin_name() const { return this->name; }
    }
}
