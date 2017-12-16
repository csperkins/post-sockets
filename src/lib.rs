// Copyright (c) 2017 University of Glasgow
// All rights reserved.
// 
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions 
// are met:
// 
// 1. Redistributions of source code must retain the above copyright 
//    notice, this list of conditions and the following disclaimer.
// 
// 2. Redistributions in binary form must reproduce the above copyright 
//    notice, this list of conditions and the following disclaimer in the 
//    documentation and/or other materials provided with the distribution.
// 
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
// THE POSSIBILITY OF SUCH DAMAGE.
//
// SPDX-License-Identifier: BSD-2-Clause
//
// ================================================================================================

struct MessageDeadline(u32);
struct MessagePriority(u32);

trait Message {
    fn is_complete(&self) -> bool;
    fn is_immediate(&self) -> bool;
    fn is_idempotent(&self) -> bool;
    fn priority(&self) -> MessagePriority;
    fn deadline(&self) -> Option<MessageDeadline>;
    fn depends_on(&self) -> Vec<&Self>;
}

trait Protocol {
    type M : Message;

    // FIXME: needs to account for partial messages
    fn decode_message(&self, data : &[u8]) -> Option<Self::M>;
    fn encode_message(&self, &Self::M) -> [u8];
}

// ================================================================================================

struct Carrier<P : Protocol> {
}

impl<P : Protocol> Carrier<P> {
    fn send(&mut self, msg : &P::M) {
        unimplemented!()
    }
}

// ================================================================================================

struct Listener<P : Protocol> {
}

impl<P : Protocol> Listener<P> {
    fn listen() -> Self {
    }

    fn accept(&mut self) -> Carrier<P> {
    }
}

// ================================================================================================
