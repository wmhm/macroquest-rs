#pragma once
#include "rust/cxx.h"
#pragma warning(push)
#pragma warning(disable : 4100 4189 4201 4245 4458)
#include "eqlib/EQLib.h"
#pragma warning(pop)

namespace mqrust
{
    namespace eqlib
    {
        class EQGroundItem : ::eqlib::EQGroundItem
        {
        public:
            rust::Str name() const;
        };
    }
}
