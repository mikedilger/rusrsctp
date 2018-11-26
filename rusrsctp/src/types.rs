
use rusrsctp_sys::*;

#[derive(Debug, Copy, Clone)]
pub enum Shutdown {
    Rd,
    Wr,
    RdWr
}

bitflags! {
    pub struct MsgFlags: u32 {
        const OOB = MSG_OOB;
        const PEEK = MSG_PEEK;
        const DONTROUTE = MSG_DONTROUTE;
        const CTRUNC = MSG_CTRUNC;
        const PROXY = MSG_PROXY;
        const TRUNC = MSG_TRUNC;
        const DONTWAIT = MSG_DONTWAIT;
        const EOR = MSG_EOR;
        const WAITALL = MSG_WAITALL;
        const FIN = MSG_FIN;
        const SYN = MSG_SYN;
        const CONFIRM = MSG_CONFIRM;
        const RST = MSG_RST;
        const ERRQUEUE = MSG_ERRQUEUE;
        const NOSIGNAL = MSG_NOSIGNAL;
        const MORE = MSG_MORE;
        const WAITFORONE = MSG_WAITFORONE;
        const BATCH = MSG_BATCH;
        const ZEROCOPY = MSG_ZEROCOPY;
        const FASTOPEN = MSG_FASTOPEN;
        const CMSG_CLOEXEC = MSG_CMSG_CLOEXEC;
    }
}

bitflags! {
    pub struct SctpFlags: u16 {
        /// This value indicates that the data was never put on the wire.
        const DATA_UNSENT = SCTP_DATA_UNSENT as u16;
        /// This value indicates that the data was put on the
        /// wire.  Note that this does not necessarily mean that the data
        /// was (or was not) successfully delivered.
        const DATA_SENT = SCTP_DATA_SENT as u16;
        /// For nxt_flags: This flag is present when the next message is
        /// not a user message but instead is a notification.
        const NOTIFICATION = SCTP_NOTIFICATION as u16;
        /// for nxt_flags: This flag indicates that the entire message has
        /// been received and is in the socket buffer.  Note that this has
        /// special implications with respect to the nxt_length field; see
        /// the description for nxt_length.
        const COMPLETE = SCTP_COMPLETE as u16;
        /// Setting this flag invokes the SCTP graceful shutdown procedures on the
        /// specified association. Graceful shutdown assures that all data queued
        /// by both endpoints is successfully transmitted before closing the
        /// association.
        const EOF = SCTP_EOF as u16;
        /// Setting this flag causes the specified association to abort by
        /// sending an ABORT message to the peer. The ABORT chunk will contain
        /// an error cause of 'User Initiated Abort' with cause code 12.
        /// The cause-specific information of this error cause is provided
        /// in msg_iov.
        const ABORT = SCTP_ABORT as u16;
        /// on send: This flag requests unordered delivery of the message. If the
        /// flag is clear, the datagram is considered an ordered send.
        /// on receive: This flag is present when the message was sent
        /// unordered
        const UNORDERED = SCTP_UNORDERED as u16;
        /// This flag, for a one-to-many style socket, requests that the SCTP
        /// stack override the primary destination address with the address found
        /// with the send call
        const ADDR_OVER = SCTP_ADDR_OVER as u16;
        /// This flag, if set, will cause a one-to-many style socket to send the
        /// message to all associations that are currently established on this
        /// socket.  For the one-to-one style socket, this flag has no effect.
        const SENDALL = SCTP_SENDALL as u16;
        /// When using the EXPLICIT_EOR option, this flag explicitly marks
        /// this data as the end of the message, and if off, means more data
        /// is forthcoming within the same message.
        const EOR = SCTP_EOR as u16;
        /// If this bit is set, a SACK is sent immediately, rather than waiting
        /// for thie SACK timer to expire.
        const SACK_IMMEDIATELY = SCTP_SACK_IMMEDIATELY as u16;
    }
}
