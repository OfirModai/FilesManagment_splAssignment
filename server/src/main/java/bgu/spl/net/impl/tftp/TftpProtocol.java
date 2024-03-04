package bgu.spl.net.impl.tftp;

import bgu.spl.net.Util;
import bgu.spl.net.api.BidiMessagingProtocol;
import bgu.spl.net.srv.BaseConnections;
import bgu.spl.net.srv.BlockingConnectionHandler;
import bgu.spl.net.srv.Connections;
import com.sun.org.apache.xml.internal.security.algorithms.implementations.IntegrityHmac;
import com.sun.tools.jdi.Packet;

import java.net.DatagramPacket;
import java.net.Socket;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.Hashtable;

public class TftpProtocol implements BidiMessagingProtocol<byte[]> {

    private boolean shouldTerminate = false;
    private BaseConnections<byte[]> connections;
    private int ownerId;

    public TftpProtocol(int connectionId, Connections<byte[]> connections) {
        start(connectionId, connections);
    }

    @Override
    public void start(int connectionId, Connections<byte[]> connections) {
        this.connections = (BaseConnections<byte[]>) connections;
        this.ownerId = connectionId;
    }

    @Override
    public byte[] process(byte[] message) {
        byte type = message[1];
        byte[] data = new byte[message.length - 2];
        System.arraycopy(message, 2, data, 0, data.length);
        switch (type) {
            case 1:
            case 2:
            case 3:
            case 4:
            case 5:
            case 6:
            case 7:
                LogRQ(data);
            case 8:
            case 9:
            case 0x10:
        }
        return null;
    }

    @Override
    public boolean shouldTerminate() {
        shouldTerminate = true;
        // TODO implement this
        throw new UnsupportedOperationException("Unimplemented method 'shouldTerminate'");
    }

    // message is after decode
    private Packet LogRQ(byte[] message) {
        String userName = new String(message);
        int connectionId = userName.hashCode();
        Packet packet = null;
        if (connections.canConnect(connectionId)) {
            BlockingConnectionHandler<byte[]> blockingConnectionHandler = new BlockingConnectionHandler<>(
                    new Socket(), new TftpEncoderDecoder(), new TftpProtocol(connectionId, connections));
            connections.connect(connectionId, blockingConnectionHandler);
            // ACK packet
        } else {
            // ERROR packet
        }
        return packet;
    }

    private static byte[] getPartArray(byte[] src, int part) {
        int maxLength = 512;
        int copyLength = maxLength;
        // for last part \ src.length < maxLength - we copy only the appropriate size.
        if (!(src.length - maxLength * part > maxLength))
            copyLength = src.length - maxLength * part;
        byte[] retByte = new byte[copyLength];
        System.arraycopy(src, part * maxLength, retByte, 0, copyLength);
        return retByte;
    }

}
