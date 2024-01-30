#include "macroquest/include/mq.h"
#include "macroquest/src/ffi/mod.rs.h"

namespace mqrust
{
    namespace mq
    {
        void write_chat_color(rust::Str line, int color)
        {
            ::mq::WriteChatColorf("%s", color, static_cast<std::string>(line).c_str());
        }
    }
}
