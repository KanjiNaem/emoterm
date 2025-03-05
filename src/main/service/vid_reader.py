import cv2
import time
import numpy as np

if __name__ == '__main__':
    cap = cv2.VideoCapture('../../test/testMp4/qtEmoGuyDancingTest.mp4')

    #check if cam opened
    if not cap.isOpened():
        print("Error opening vid file")
        exit(0)

    mp4_w = cap.get(cv2.CAP_PROP_FRAME_WIDTH) 
    mp4_h = cap.get(cv2.CAP_PROP_FRAME_HEIGHT)
    mp4_fps = cap.get(cv2.CAP_PROP_FPS)

    print('Video properties:')
    print('Video Dimensions: ', mp4_w, mp4_h)
    print('Video FPS: ', mp4_fps)

    while(cap.isOpened()):
        returnStatus, frame = cap.read()

        if returnStatus:
            imgin = frame[:, :, (2, 1, 0)] #rbg order for some reason btw
        else:
            break
        
        print(np.array(frame).shape)
        cv2.imshow("camOut", frame)
        cv2.waitKey(0)
        # time.sleep(1 / (mp4_fps * 1.5))

    cap.release()