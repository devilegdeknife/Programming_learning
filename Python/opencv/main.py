import cv2
import os
import numpy as np
import tkinter as tk
from tkinter import messagebox
from tkinter import filedialog
from PIL import Image, ImageTk
from deepface import DeepFace
import tkinter.font as tkFont


# 定义全局变量，用于在不同函数间传递主菜单窗口的引用
main_menu_window = None


# 登录函数
def login():
    username = entry_username.get()
    password = entry_password.get()

    if username == "admin" and password == "password":  # 假设的用户名和密码
        messagebox.showinfo("登录成功", "欢迎回来，" + username)
        show_main_menu()  # 显示主菜单界面
        root.withdraw()  # 隐藏登录界面
    else:
        messagebox.showerror("登录失败", "用户名或密码错误")


# 返回登录界面
def back_to_login():
    global main_menu_window
    if main_menu_window is not None:
        main_menu_window.destroy()  # 销毁主菜单窗口
    root.deiconify()  # 重新显示登录界面


# 退出程序
def exit_program():
    global main_menu_window
    if main_menu_window:
        main_menu_window.destroy()  # 销毁主菜单窗口
    root.destroy()  # 销毁登录界面，完全退出程序


# 当用户尝试关闭主菜单窗口时调用的函数
def on_close():
    global main_menu_window
    if main_menu_window:
        main_menu_window.destroy()  # 销毁主菜单窗口
    else:
        root.destroy()  # 如果没有主菜单窗口，则销毁登录窗口


# 创建一个 VideoCapture 对象，参数 0 表示使用第一个摄像头
cap = cv2.VideoCapture(0)


def stop_video_stream():
    global cap
    # 释放摄像头资源并关闭显示窗口
    cap.release()
    cv2.destroyAllWindows()

    root.update()  # 更新Tkinter GUI
    show_main_menu()  # 返回主菜单


def detect_faces_live():
    global cap, display_area, last_frame

    try:
        while True:
            # 从摄像头读取一帧
            ret, frame = cap.read()

            # 如果读取成功，进行人脸检测和情绪分析
            if ret:
                # 使用 DeepFace.analyze 函数进行人脸检测和情绪分析
                results = DeepFace.analyze(frame, actions=['age', 'emotion', 'gender', 'race'],
                                        detector_backend="mtcnn", enforce_detection=False)

                for result in results:
                    age = result["age"]
                    emotion = result["dominant_emotion"]
                    gender = result["dominant_gender"]
                    race = result["dominant_race"]
                    region = result["region"]

                    # 绘制人脸矩形
                    cv2.rectangle(
                        frame, (region["x"], region["y"]),
                        (region["x"] + region["w"], region["y"] + region["h"]),
                        (255, 0, 0), 1
                    )
                    # 在人脸上方显示情绪、年龄、性别和种族
                    cv2.putText(frame,
                                f"age: {age}, emotion: {emotion}, gender: {gender}, race: {race}",
                                (region["x"] - 135, region["y"]),
                                cv2.FONT_HERSHEY_SIMPLEX,
                                0.6,
                                (161, 16, 203),
                                1,
                                cv2.LINE_AA)

                    # 显示画面
                    cv2.imshow('Emotion Detection', frame)

                    # 如果按下 ' ' 键，退出循环
                    if cv2.waitKey(1) & 0xFF == ord(' '):
                        break

                # 保存最后一帧
                cv2.imwrite("last_frame.jpg", frame)

    except KeyboardInterrupt:
        print("用户中断")

    finally:
        # 释放摄像头资源并关闭显示窗口
        cap.release()
        cv2.destroyAllWindows()

        root.update()  # 更新Tkinter GUI
        show_main_menu()  # 返回主菜单


def detect_faces_image():
    global main_menu_window
    if main_menu_window:
        main_menu_window.destroy()  # 关闭主菜单窗口

    # 打开一个文件对话框，让用户选择一张图片
    file_path = filedialog.askopenfilename()

    # 检查是否选择了文件
    if not file_path:
        print("没有选择文件。")
        show_main_menu()
        return

    # 使用DeepFace分析图片中的人脸情绪
    try:
        result = DeepFace.analyze(file_path, actions=['emotion'], enforce_detection=False)
    except ValueError as e:
        print("错误:", e)
        show_main_menu()
        return

    # 检查结果是否是一个字典，且包含'dominant_emotion'键
    if isinstance(result, dict) and 'dominant_emotion' in result:
        # 打印主导情绪
        print(result['dominant_emotion'])

        # 在图片上标注主导情绪并显示图片
        try:
            img = cv2.imread(file_path)
            cv2.putText(img, result['dominant_emotion'], (50, 50), cv2.FONT_HERSHEY_SIMPLEX, 1,
                        (0, 0, 255), 2, cv2.LINE_4)
            cv2.imshow('情绪检测', img)
            cv2.waitKey(0)
            cv2.destroyAllWindows()
        except Exception as e:
            print("显示图片时出错:", e)
    else:
        print("未检测到人脸。")

    # 返回主菜单
    show_main_menu()


# 主菜单界面
# 主菜单界面的函数
def show_main_menu():
    global main_menu_window  # 声明全局变量

    if main_menu_window is None or not main_menu_window.winfo_exists():  # 检查主菜单窗口是否存在
        main_menu_window = tk.Toplevel(root)  # 创建新的主菜单窗口
        main_menu_window.title("主菜单")  # 设置窗口标题
        main_menu_window.geometry('400x300')  # 设置窗口大小

        # 设置窗口关闭时的回调函数
        main_menu_window.protocol("WM_DELETE_WINDOW", on_close)

        # 创建一个现代风格的显示区域
        display_area = tk.Label(main_menu_window, text="功能展示区", bg='#e0e0e0', width=30, height=10,
                                font=tkFont.Font(family="Helvetica", size=16))
        display_area.place(relx=0.5, rely=0.3, anchor='center')  # 居中放置

        # 创建按钮并使用place方法进行绝对定位以实现现代布局
        button_live = tk.Button(main_menu_window, text="实时人脸情绪检测", command=detect_faces_live,
                                font=("Helvetica", 16), fg="#ffffff", bg="#0078d7", padx=20, pady=10,
                                borderwidth=0, relief="flat")
        button_live.place(relx=0.5, rely=0.5, anchor='center')  # 居中放置

        # 创建返回登录和退出按钮，使用grid布局管理
        button_login = tk.Button(main_menu_window, text="返回登录", command=back_to_login,
                                 font=("Helvetica", 12), fg="#333333", bg="#f0f0f0", padx=10, pady=5,
                                 borderwidth=1, relief="flat")
        button_login.grid(row=6, column=0, padx=20, pady=10, sticky='ew')

        button_exit = tk.Button(main_menu_window, text="退出程序", command=exit_program,
                                font=("Helvetica", 12), fg="#333333", bg="#f0f0f0", padx=10, pady=5,
                                borderwidth=1, relief="flat")
        button_exit.grid(row=6, column=1, padx=20, pady=10, sticky='ew')

        # 设置窗口的最小尺寸
        main_menu_window.minsize(300, 200)


# 创建登录窗口
root = tk.Tk()
root.title("登录窗口")

# 创建用户名和密码输入框（保持不变）
label_username = tk.Label(root, text="用户名:")
entry_username = tk.Entry(root)
label_username.grid(row=0, column=0, padx=10, pady=10)
entry_username.grid(row=0, column=1, padx=10)

label_password = tk.Label(root, text="密码:")
entry_password = tk.Entry(root, show="*")
label_password.grid(row=1, column=0, padx=10, pady=10)
entry_password.grid(row=1, column=1, padx=10)

# 创建登录按钮
tk.Button(root, text="登录", command=login).grid(row=2, column=0, columnspan=2, pady=10)

# 运行主循环
root.mainloop()
