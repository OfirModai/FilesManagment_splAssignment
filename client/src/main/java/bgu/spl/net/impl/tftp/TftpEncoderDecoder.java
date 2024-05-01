package bgu.spl.net.impl.tftp;

import bgu.spl.net.api.MessageEncoderDecoder;

import java.util.LinkedList;
import java.util.List;

public class TftpEncoderDecoder implements MessageEncoderDecoder<String> {
    private List<Byte> bytes = new LinkedList<>();

    @Override
    public byte[] decodeNextByte(byte nextByte) {
        bytes.add(nextByte);
        if (bytes.size() <= 1) return null;
        if (bytes.size() == 2 & bytes.get(1) == 0) {
            bytes.remove(1);
            return null;
        }
        byte[] retBytes = null;
        int op = bytes.get(1);

        if (op > 0xa | op < 1)
            throw new IllegalArgumentException("OP not valid");
        switch (op) { //this order is important!
            case 3:
                // data packet
                if (bytes.size() >= 4 &&
                        bytes.size() - 6 == Util.convertBytesToShort(bytes.get(2), bytes.get(3)))
                    retBytes = Util.convertListToArray(bytes);
                break;
            case 4:
                if (bytes.size() == 4) retBytes = Util.convertListToArray(bytes);
                break;
            case 6:
            case 0xa:
                retBytes = Util.convertListToArray(bytes);
                break;
            case 9: // bCast
                if (bytes.size() >= 4 && bytes.get(bytes.size() - 1) == 0)
                    retBytes = Util.convertListToArray(bytes);
                break;
            case 5:
                if (bytes.size() < 5) break;
            default:
                if (bytes.get(bytes.size() - 1) == 0 | bytes.size() - 6 == Util.MAX_PACKET_LENGTH)
                    retBytes = Util.convertListToArray(bytes);
                break;
        }
        if (retBytes != null) bytes.clear();
        return retBytes;
    }

    @Override
    public byte[] encode(String message) throws IllegalArgumentException {
        byte[] encoded;
        int index = message.indexOf(' ');
        // this calc gets the first word
        switch ((index != -1) ? message.substring(0, index) : message) {
            case "LOGRQ":
                encoded = getSimplePacket(7,
                        message.substring(index + 1).getBytes());
                break;
            case "DELRQ":
                encoded = getSimplePacket(8,
                        message.substring(index + 1).getBytes());
                break;
            case "RRQ":
                encoded = getSimplePacket(1,
                        message.substring(index + 1).getBytes());
                break;
            case "WRQ":
                encoded = getSimplePacket(2,
                        message.substring(index + 1).getBytes());
                break;
            case "DIRQ":
                encoded = new byte[]{0, 6};
                break;
            case "DISC":
                encoded = new byte[]{0, 0xa};
                break;
            default:
                throw new IllegalArgumentException();
        }
        return encoded;
    }

    private byte[] getSimplePacket(int opcode, byte[] name) {
        return Util.concatArrays(new byte[]{0, (byte) opcode}, name, new byte[]{0});
    }

}