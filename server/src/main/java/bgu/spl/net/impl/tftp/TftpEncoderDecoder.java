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
        if (nextByte == 0) {
            bytes.add(nextByte);
            return Util.convertListToArray(bytes);
        }
        bytes.add(nextByte);
        return null;
    }

    @Override
    public byte[] encode(byte[] message) {
        return (message + "0").getBytes(StandardCharsets.UTF_8);
    }

}