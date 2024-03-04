package bgu.spl.net.impl.tftp;

import bgu.spl.net.api.MessageEncoderDecoder;

import java.nio.charset.StandardCharsets;
import java.util.LinkedList;
import java.util.List;

import bgu.spl.net.Util;

public class TftpEncoderDecoder implements MessageEncoderDecoder<byte[]> {
    private List<Byte> bytes = new LinkedList<>();

    @Override
    public byte[] decodeNextByte(byte nextByte) {
        bytes.add(nextByte);
        byte[] retBytes = null;
        if (nextByte == 0 | bytes.size() == 512) { //should end msg
            retBytes = Util.convertListToArray(bytes);
            bytes = new LinkedList<>();
        }
        return retBytes;
    }

    @Override
    public byte[] encode(byte[] message) throws IllegalArgumentException {
        if (message.length >= 512)
            throw new IllegalArgumentException ("msg over 512 bytes");
        byte[] resByte = new byte[message.length + 1];
        System.arraycopy(message, 0, resByte, 0, message.length);
        resByte[resByte.length - 1] = 0;
        return resByte;
    }

}