#pragma once
#include "rust/cxx.h"
#pragma warning(push)
#pragma warning(disable : 4100 4189 4201 4245 4458)
#include "mq/Plugin.h"
#pragma warning(pop)

namespace mqrust
{
    namespace mq
    {
        // Path Functions
        rust::Str get_path_MQRoot();
        rust::Str get_path_Config();
        rust::Str get_path_MQini();
        rust::Str get_path_Macros();
        rust::Str get_path_Logs();
        rust::Str get_path_CrashDumps();
        rust::Str get_path_Plugins();
        rust::Str get_path_Resources();
        rust::Str get_path_EverQuest();

        // General Functions
        void write_chat_color(rust::Str line, int color);
    }
}
