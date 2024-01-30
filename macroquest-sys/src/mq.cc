#include "macroquest-sys/include/mq.h"
#include "macroquest-sys/src/lib.rs.h"

namespace mqrust
{
    namespace mq
    {
        void write_chat_color(rust::Str line, int color)
        {
            ::mq::WriteChatColor(static_cast<std::string>(line).c_str(), color);
        }
    }
}
