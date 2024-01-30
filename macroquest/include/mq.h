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
        void write_chat_color(rust::Str line, int color);
    }
}
